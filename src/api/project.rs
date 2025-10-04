// TODO: add to project in cargo
// use actix_web::{get, web, App, HttpServer, Result};

pub type Projects = Response<Projects>; 

// pub const MEMBERS: usize = 4;

pub struct Project {
    pub title: String,
    pub desc: String,
    pub comparisons: usize,
    pub member_ids: Vec<String>, 
    pub project_id: String,
}

impl Project {
    pub fn new(title: String, 
               desc: String, 
               comparisons: usize,
               member_ids: Vec<String>, 
               project_id: String) -> Self 
    {
        Self {
            title,
            desc,
            comparisons,
            member_ids,
            project_id,
        }
    }
}

fn new_from_db(project_id: String) -> Project {
    // TODO("Interface with database to get project details");

    Project{
        title: String::new(),
        desc: String::new(),
        comparisons: 0,
        member_ids: Vec::new(),
        project_id,
    }
}

pub type Tweets = Response<Tweet>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Tweet {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
            likes: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectRequest {
    pub project_id: String,
}

impl ProjectRequest {
    pub fn to_project(&self) -> Option<Project> {
        match &self.project_id {
          //If request had a project_id, return a new project generating an id
          //else return None
            Some(project_id) => Some(Project::new(project_id.to_string())),
            None => None,
        }
    }
}

// GET /pair/project
#[get("/pair/project/{project_id}")]
pub async fn getProj( path: web::Path<(u32, String)> ) -> HttpResponse {
    // TODO find the last 50 tweets and return them
    let project_id = path.into_inner();
    

    // TODO Return JSON on project 
    Ok(format!("Welcome {}, user_id {}!", friend, user_id));
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(new_from_db(project_id.1))
}

// GET /pair/pair
#[get("/pair/project/")]
pub async fn getProjPair( ) -> HttpResponse {}

// POST /pair/update
// TODO determine input
#[post("/pair/update")]
pub async fn updatePair( path: web::Path<(u32, String)> ) -> HttpResponse {}

// GET /pair/project-result
#[get("/pair/project-result/{project_id}")]
pub async fn getProjRes( path: web::Path<(u32, String)> ) -> HttpResponse {}

// GET /pair/results
#[get("/pair/results")]
pub async fn getRes( ) -> HttpResponse {}
