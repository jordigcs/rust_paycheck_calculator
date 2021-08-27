use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args[0].contains('/') || args[0].contains('\\') {
        args.remove(0);
    }

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

            if time_interval_int[1] < time_interval_int[0] {
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
    println!("Total Hours: {:.1}\nHourly Pay: {:.2}\nTotal Pay without taxes: {:.2}\n_____________\nSocial Security Tax: {:.2}\nMedicare Tax: {:.2}\nFederal Witholding: {:.2}\nFinal Total: {:.2}", total_hours, wage, pay, soc_tax, med_tax, fed_tax, pay-fed_tax-soc_tax-med_tax);

}
