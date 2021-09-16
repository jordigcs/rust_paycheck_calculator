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
#[commands(clear)]
// #[commands(calc_tax)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    
    async fn message(&self, ctx: Context, msg: Message) {

    }

}

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

// #[command]
// async fn calc_tax(ctx: &Context, msg: &Message) -> CommandResult {
//     let mut args: Vec<&str> = msg.content.split(" ").collect();
//     if args.len() > 1 {
//         args.remove(0);
//         let amount:f32 = 0;

//         let mut is_numeric:bool = true;
//         for i in args[0].chars() {
//             if !i.is_numeric() {
//                 is_numeric = false;
//             }
//         }

//         if !is_numeric {
//             msg.channel_id.send_message(&ctx.http, |m| {
//                 m.content("Value provided is not a number! Provide an amount to calculate taxes on.");
//                 m
//             }).await;
//             return
//         }
//         else {
//             amount = args[0].parse::<f32>().unwrap()
//         }
//         msg.channel_id.send_message(&ctx.http, |m| {
//             let soc_tax = amount*(6.02/100.0);
//             let med_tax = amount*(1.45/100.0);
//             m.content(format!("Social Security Tax: {:?}\nMedicare Tax: {:?}\nRemaning amount: {:?}", soc_tax, med_tax, amount-soc_tax-med_tax));
//             m
//         }).await;
//         Ok(())
//     }
// }

#[command]
async fn clear(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args: Vec<&str> = msg.content.split(" ").collect();
    if args.len() > 1 {
        args.remove(0);
        let amount_to_clear:i8 = 0;

        let mut is_numeric:bool = true;
        for i in args[0].chars() {
            if !i.is_numeric() {
                is_numeric = false;
            }
        }

        if !is_numeric {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.content("Value provided is not a number! Provide a number for messages to delete.");
                m
            }).await;
            return
        }
        else {
            amount_to_clear = args[0].parse::<i8>().unwrap()
        }
        msg.channel_id.messages(ctx, |messages| {
            println!("{:?}", messages);
            // for i in 0..amount_to_clear {
            //     msg.channel_id.delete_message(ctx, messages[i]);
            // }
            messages
        }).await;
        msg.channel_id.send_message(&ctx.http, |m| {
            m.content(format!("Deleted {:?} messages.", amount_to_clear));
            m
        }).await;
    }
    Ok(())
}

#[command]
async fn hours(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args: Vec<&str> = msg.content.split(" ").collect();
    if args.len() > 1 {
        args.remove(0);

        let mut total_hours: i8 = 0;
        let mut wage:f32 = 8.5;
        let mut share_publicly:bool = false;
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
                match arg.to_lowercase().as_str() {
                    "public" => {
                        println!("public {:?}", arg);
                        share_publicly = true;
                    }
                    _ => {
                        println!("Wage {:?}", arg);
                        wage = arg.parse::<f32>().unwrap();
                    }
                }
            }
        }

        let pay = f32::from(total_hours)*wage;
        let soc_tax = pay*(6.02/100.0);
        let med_tax = pay*(1.45/100.0);
        //println!("Total Hours: {:.1}\nHourly Pay: {:.2}\nTotal Pay without taxes: {:.2}\n_____________\nSocial Security Tax: {:.2}\nMedicare Tax: {:.2}\nFederal Witholding: {:.2}\nFinal Total: {:.2}", total_hours, wage, pay, soc_tax, med_tax, fed_tax, pay-fed_tax-soc_tax-med_tax);

        let colors : Vec<i32> = vec![0x008C15, 0xFFC600];

        let orig_msg = msg;
        let id;
        if !share_publicly {
            let dm = msg.author.create_dm_channel(ctx).await?;
            id = dm.id;
        }
        else {
            id = msg.channel_id;
        }
        let msg2 = id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("Hour + Pay Calculator");
                        e.color(*colors.choose(&mut rand::thread_rng()).unwrap());
                        e.description("Calculates how many hours you work and around how much you will make without tips.");
                        e.fields(vec![
                            ("Gross Pay", format!("Total Hours: {:.1}\nHourly Pay: {:.2}\nTotal Pay without taxes: {:.2}", total_hours, wage, pay), true),
                            ("Net Pay", format!("Social Security Tax: {:.2}\nMedicare Tax: {:.2}\n**Final Total: {:.2}**", soc_tax, med_tax, pay-soc_tax-med_tax), true),
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

            if let Err(why) = msg2 {
                println!("Error sending message: {:?}", why);
            }
            else {
                if !share_publicly {
                orig_msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("Hour + Pay Calculator");
                        e.color(*colors.choose(&mut rand::thread_rng()).unwrap());
                        e.description("Results sent! Check DMs :D");

                        e
                    });
                    m
                })
                .await?;
            }
            }
        }
    else {
        msg.reply(ctx, "Format invalid. Here's an example of how to use this command:\n`~hours 4-cl 5-cl 8-cl 11-4 8-5`\nIf you want to change the hourly wage used to calculate pay, just add your wage at the end of the command\n`~hours 4-cl 5-cl 8-cl 11-4 8-5 9.50`").await?;
    }
    Ok(())
}
