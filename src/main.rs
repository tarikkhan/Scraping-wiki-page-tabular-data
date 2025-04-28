use scraper::{Html, Selector};
//use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};
use wiki_scrap::{DataList, SingleCompanyDetails};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        "https://en.wikipedia.org/wiki/List_of_largest_companies_in_the_United_States_by_revenue";

    search_wiki_generate_data_files(url).await?;

    Ok(())
}

async fn search_wiki_generate_data_files(
    url: &str,
) -> Result<Vec<SingleCompanyDetails>, Box<dyn std::error::Error>> {
    let mut list_of_single_company_details = Vec::new();

    let res = reqwest::get(url).await?;

    if !res.status().is_success() {
        println!("{}", res.status());
        let error_msg = format!("Request failed with status code: {}", res.status());
        return Err(error_msg.into());
    }

    let res = res.text().await?;
    let document = Html::parse_document(&res);

    let selector = Selector::parse("table.wikitable.sortable tbody tr")?;
    let td_selector = Selector::parse("td")?;

    let mut tr_list = Vec::new();

    let trs = document.select(&selector);
    for tr in trs {
        let mut td_list = Vec::new();
        let tds = tr.select(&td_selector);
        for td in tds {
            td_list.push(td.text().collect::<String>().trim().to_owned()); // contains all the td values in a single row
        }
        tr_list.push(td_list);
    }

    for item in &tr_list {
        if item.len() == 7 {
            let list_item = SingleCompanyDetails::new(
                item[0].clone(),
                item[1].clone(),
                item[2].clone(),
                item[3].clone(),
                item[4].clone(),
                item[5].clone(),
                item[6].clone(),
            );
            list_of_single_company_details.push(list_item);
        }
    }

    let mut rank = Vec::new();
    let mut name = Vec::new();
    let mut industry = Vec::new();
    let mut revenue_usd_millions = Vec::new();
    let mut revenue_growth = Vec::new();
    let mut employees = Vec::new();
    let mut headquarters = Vec::new();

    for item in &tr_list {
        if item.len() == 7 {
            rank.push(item[0].clone());
            name.push(item[1].clone());
            industry.push(item[2].clone());
            revenue_usd_millions.push(item[3].clone());
            revenue_growth.push(item[4].clone());
            employees.push(item[5].clone());
            headquarters.push(item[6].clone());
        }
    }

    let company_data_list = DataList::new(
        rank,
        name,
        industry,
        revenue_usd_millions,
        revenue_growth,
        employees,
        headquarters,
    );

    // all data is saved in a single struct
    let serialized = serde_json::to_string(&company_data_list).unwrap();
    let mut json_file = File::create("data_list.json")?;
    json_file.write_all(serialized.as_bytes())?;

    // each individual company data is saved in a struct, then we have a vector of those structs
    let serialized = serde_json::to_string(&list_of_single_company_details).unwrap();
    let mut json_file = File::create("company_list.json")?;
    json_file.write_all(serialized.as_bytes())?;

    let csv_file = File::create("company_list.csv")?;

    let mut wtr = csv::Writer::from_writer(csv_file);

    for company in &list_of_single_company_details {
        wtr.serialize(company)?;
        wtr.flush()?;
    }

    Ok(list_of_single_company_details)
}
