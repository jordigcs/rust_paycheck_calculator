use rand::seq::SliceRandom;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
use std::env;

#[group]
#[commands(hours)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn hours(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args: Vec<&str> = msg.content.split(" ").collect();
    if args.len() > 1 {
        args.remove(0);

        let mut total_hours: i8 = 0;
        let mut wage:f32 = 8.5;
        for arg in args.iter() {
            if arg.contains('-') {
                let mut time_interval_str : Vec<&str> = arg.split("-").collect();

                let mut time_interval_int : Vec<i8> = vec!();
        
                if time_interval_str.len() > 1 {
                    if time_interval_str[1].to_lowercase() == "cl" {
                        time_interval_str[1] = "10";
                    }
                }

                for (ind, _value) in time_interval_str.iter().enumerate() {
                    let num : i8 = time_interval_str[ind].parse().unwrap();
                    time_interval_int.push(num);
                }

                if time_interval_int[1] <= time_interval_int[0] {
                    total_hours += (12-time_interval_int[0]) + time_interval_int[1];
                }
                else {
                    total_hours += time_interval_int[1] - time_interval_int[0];
                }
            }
            else {
                wage = arg.parse::<f32>().unwrap();
            }
        }

        let pay = f32::from(total_hours)*wage;
        let soc_tax = pay*(6.02/100.0);
        let med_tax = pay*(1.45/100.0);
        let fed_tax = 10.00;
        //println!("Total Hours: {:.1}\nHourly Pay: {:.2}\nTotal Pay without taxes: {:.2}\n_____________\nSocial Security Tax: {:.2}\nMedicare Tax: {:.2}\nFederal Witholding: {:.2}\nFinal Total: {:.2}", total_hours, wage, pay, soc_tax, med_tax, fed_tax, pay-fed_tax-soc_tax-med_tax);

        let colors : Vec<i32> = vec![0x008C15, 0xFFC600];


        println!("{}", msg.content);
        let channel = msg.author.create_dm_channel(&ctx).await;
        match channel {
            Ok(c) => {
                let r = c.id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("Hour + Pay Calculator");
                        e.color(*colors.choose(&mut rand::thread_rng()).unwrap());
                        e.description("Calculates how many hours you work and around how much you will make without tips.");
                        e.fields(vec![
                            ("Gross Pay", format!("Total Hours: {:.1}\nHourly Pay: {:.2}\nTotal Pay without taxes: {:.2}", total_hours, wage, pay), true),
                            ("Net Pay", format!("Social Security Tax: {:.2}\nMedicare Tax: {:.2}\nFederal Witholding: {:.2}\n**Final Total: {:.2}**", soc_tax, med_tax, fed_tax, pay-fed_tax-soc_tax-med_tax), true),
                        ]);
                        e.footer(|f| {
                            f.text("a small little coding project by Jordi :)");

                            f
                        });

                        e
                    });
                    m
                })
                .await;

                match r{
                    Ok(_) => {
                        c.id.send_message(&ctx.http, |f| {
                        f.embed(|e| {
                            e.title("Hour + Pay Calculator");
                            e.color(*colors.choose(&mut rand::thread_rng()).unwrap());
                            e.description("Result sent! Check your DMs! :D");
                            e
                        });
                        f
                    }).await?;
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }

        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
    }
    else {
        msg.reply(ctx, "Format invalid. Here's an example of how to use this command:\n`~hours 4-cl 5-cl 8-cl 11-4 8-5`\nIf you want to change the hourly wage used to calculate pay, just add your wage at the end of the command\n`~hours 4-cl 5-cl 8-cl 11-4 8-5 9.50`").await?;
    }
    Ok(())
}
