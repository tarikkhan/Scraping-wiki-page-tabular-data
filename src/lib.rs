
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SingleCompanyDetails {
    rank: String,
    name: String,
    industry: String,
    revenue_usd_millions: String,
    revenue_growth: String,
    employees: String,
    headquarters: String,
}

impl SingleCompanyDetails {
    pub fn new(
        rank: String,
        name: String,
        industry: String,
        revenue_usd_millions: String,
        revenue_growth: String,
        employees: String,
        headquarters: String,
    ) -> Self {
        SingleCompanyDetails {
            rank,
            name,
            industry,
            revenue_usd_millions,
            revenue_growth,
            employees,
            headquarters,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DataList {
    rank: Vec<String>,
    name: Vec<String>,
    industry: Vec<String>,
    revenue_usd_millions: Vec<String>,
    revenue_growth: Vec<String>,
    employees: Vec<String>,
    headquarters: Vec<String>,
}

impl DataList {
    pub fn new(
        rank: Vec<String>,
        name: Vec<String>,
        industry: Vec<String>,
        revenue_usd_millions: Vec<String>,
        revenue_growth: Vec<String>,
        employees: Vec<String>,
        headquarters: Vec<String>,
    ) -> Self {
        DataList {
            rank,
            name,
            industry,
            revenue_usd_millions,
            revenue_growth,
            employees,
            headquarters,
        }
    }
}
