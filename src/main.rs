use std::str::FromStr;

use anyhow::Result;
use entity::{group, user};
use ferrischat::{
    choose::{Group, Register, select_enum},
    key::{hash_password, verify_password},
    message::{Message, MessageBody},
    reception::subscribe_loop,
    ticket::Ticket,
    user::create_user,
};
use iroh::{Endpoint, SecretKey, protocol::Router};
use iroh_gossip::{Gossip, TopicId};
use migration::Migrator;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, FromQueryResult, InsertResult, IntoActiveModel,
    QueryFilter, QuerySelect, RelationTrait, TryIntoModel,
};
use sea_orm::{Database, DbErr};
use sea_orm_migration::prelude::*;
fn input_loop(line_tx: tokio::sync::mpsc::Sender<String>) -> Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    loop {
        stdin.read_line(&mut buffer)?;
        line_tx.blocking_send(buffer.clone())?;
        buffer.clear();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::connect("sqlite://./data.db").await?;

    Migrator::up(&db, None).await.map_err(|e| {
        eprintln!("Migration failed: {:?}", e);
        DbErr::Migration(format!("migration failed: {:?}", e))
    })?;

    println!("migrations applied, starting app…");

    let endpoint = Endpoint::bind().await?;
    println!("> our endpoint id: {}", endpoint.id());
    let gossip = Gossip::builder().spawn(endpoint.clone());

    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn();

    let user = match select_enum(
        "Do you want to register or login",
        vec![Register::Register, Register::Login],
    )? {
        Register::Login => {
            let phone_str = inquire::Text::new("Enter phone number:").prompt()?;
            let phone: u64 = phone_str.parse()?;
            if let Some(user) = entity::user::Entity::find()
                .filter(entity::user::Column::PhoneNo.eq(phone))
                .one(&db)
                .await?
            {
                println!("User exists: {}", user.name);
                let result = loop {
                    let p = inquire::Password::new("pwd").prompt()?;
                    if verify_password(&user.password, &p) {
                        break user;
                    }
                    match select_enum(
                        "Retry or Register?",
                        vec![Register::Register, Register::Retry],
                    )? {
                        Register::Retry => {
                            continue;
                        }
                        Register::Register => break create_user(db.clone()).await?,
                        _ => unreachable!(),
                    }
                };
                result
            } else {
                println!("❌ No user found with this number. Please register");
                create_user(db.clone()).await?
            }
        }
        Register::Register => create_user(db.clone()).await?,
        _ => unreachable!(),
    };
    #[derive(Debug, FromQueryResult)]
    struct GroupWithOwner {
        pub group_name: String,
        pub group_ticket: String,
        pub owner_name: String,
    }

    let (topic, endpoints) = match select_enum(
        "Do you want to create or join a group",
        vec![Group::Create, Group::Join],
    )? {
        Group::Create => {
            let group_name = inquire::Text::new("Enter your group name:").prompt()?;
            let topic = TopicId::from_bytes(rand::random());
            println!("> opening chat room for topic {topic}");
            let ticket = {
                let me = endpoint.addr();
                let endpoints = vec![me];
                Ticket { topic, endpoints }
            };
            println!("> ticket to join us: {ticket}");
            let group_model = group::ActiveModel {
                name: Set(group_name),
                ticket: Set(ticket.to_string()),
                owner: Set(user.id),
                ..Default::default()
            };
            group::Entity::insert(group_model.into_active_model())
                .exec(&db)
                .await?;

            (topic, ticket.clone().endpoints)
        }
        Group::Join => {
            let group_name =
                inquire::Text::new("Enter the name if the group you want to join").prompt()?;
            let groups: Vec<GroupWithOwner> = group::Entity::find()
                .filter(group::Column::Name.eq(group_name.clone()))
                .join(JoinType::InnerJoin, group::Relation::User.def())
                .select_only()
                .column_as(group::Column::Name, "group_name")
                .column_as(group::Column::Ticket, "group_ticket")
                .column_as(user::Column::Name, "owner_name")
                .into_model::<GroupWithOwner>()
                .all(&db)
                .await?;
            let group = if groups.len() > 1 {
                let owners = groups.iter().map(|g| &g.owner_name).collect();
                let selected_owner = select_enum(
                    "Many such groups exists who is the owner of your group",
                    owners,
                )?;
                let selected_group: Vec<&GroupWithOwner> = groups
                    .iter()
                    .filter(|g| &g.owner_name == selected_owner)
                    .collect();
                selected_group[0]
            } else {
                &groups[0]
            };
            let Ticket { topic, endpoints } = Ticket::from_str(&group.group_ticket.as_str())?;
            (topic, vec![])
        }
    };

    let endpoint_ids = endpoints.iter().map(|p| p.id).collect();
    if endpoints.is_empty() {
        println!("> waiting for endpoints to join us...");
    } else {
        println!("> trying to connect to {} endpoints...", endpoints.len());
    };
    let (sender, receiver) = gossip
        .subscribe_and_join(topic, endpoint_ids)
        .await?
        .split();
    println!("> connected!");

    let message = Message::new(MessageBody::AboutMe {
        from: endpoint.id(),
        name: user.name,
    });
    sender.broadcast(message.to_vec()?.into()).await?;

    tokio::spawn(subscribe_loop(receiver));

    let (line_tx, mut line_rx) = tokio::sync::mpsc::channel(1);
    std::thread::spawn(move || input_loop(line_tx));

    println!("> type a message and hit enter to broadcast...");
    while let Some(text) = line_rx.recv().await {
        let message = Message::new(MessageBody::Message {
            from: endpoint.id(),
            text: text.clone(),
        });

        sender.broadcast(message.to_vec()?.into()).await?;

        println!("> sent: {text}");
    }
    router.shutdown().await?;

    Ok(())
}
