use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct CallbackOidcRequest {
    pub code: Option<String>,
}

#[derive(Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct CallbackGithubRequest {
    pub code: Option<String>,
}

/*
curl -L \
-H "Accept: application/vnd.github+json" \
-H "Authorization: Bearer gho_E5VkThQhOygKDwfGLEsolKxR58uhxxxxxxxx" \
-H "X-GitHub-Api-Version: 2022-11-28" \
https://api.github.com/user
{
    "login": "wl4g",
    "id": 29530154,
    "node_id": "MDQ6VXNlcjI5NTMwMTU0",
    "avatar_url": "https://avatars.githubusercontent.com/u/29530154?v=4",
    "gravatar_id": "",
    "url": "https://api.github.com/users/wl4g",
    "html_url": "https://github.com/wl4g",
    "followers_url": "https://api.github.com/users/wl4g/followers",
    "following_url": "https://api.github.com/users/wl4g/following{/other_user}",
    "gists_url": "https://api.github.com/users/wl4g/gists{/gist_id}",
    "starred_url": "https://api.github.com/users/wl4g/starred{/owner}{/repo}",
    "subscriptions_url": "https://api.github.com/users/wl4g/subscriptions",
    "organizations_url": "https://api.github.com/users/wl4g/orgs",
    "repos_url": "https://api.github.com/users/wl4g/repos",
    "events_url": "https://api.github.com/users/wl4g/events{/privacy}",
    "received_events_url": "https://api.github.com/users/wl4g/received_events",
    "type": "User",
    "site_admin": false,
    "name": "Mr.James Wong",
    "company": "UNASCRIBED",
    "blog": "https://blogs.wl4g.com",
    "location": "China",
    "email": null,
    "hireable": null,
    "bio": "https://gitee.com/wl4g",
    "twitter_username": null,
    "public_repos": 47,
    "public_gists": 0,
    "followers": 19,
    "following": 9,
    "created_at": "2017-06-19T03:34:47Z",
    "updated_at": "2024-03-10T16:30:37Z"
}
*/
#[derive(Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct GithubUserInfo {
    pub id: Option<String>,
    pub login: Option<String>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub user_type: Option<String>,
    pub site_admin: Option<bool>,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<String>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: Option<i32>,
    pub public_gists: Option<i32>,
    pub followers: Option<i32>,
    pub following: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl GithubUserInfo {
    pub fn default(id: Option<String>, login: Option<String>) -> GithubUserInfo {
        GithubUserInfo {
            id: id,
            login: login,
            node_id: None,
            avatar_url: None,
            gravatar_id: None,
            url: None,
            html_url: None,
            followers_url: None,
            following_url: None,
            gists_url: None,
            starred_url: None,
            subscriptions_url: None,
            organizations_url: None,
            repos_url: None,
            events_url: None,
            received_events_url: None,
            user_type: None,
            site_admin: None,
            name: None,
            company: None,
            blog: None,
            location: None,
            email: None,
            hireable: None,
            bio: None,
            twitter_username: None,
            public_repos: None,
            public_gists: None,
            followers: None,
            following: None,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct LogoutRequest {
    //pub auth_type: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

// #[async_trait]
// impl<S> FromRequest<S> for LogoutRequest where S: Send + Sync, AppState: FromRef<S> {
//     type Rejection = (StatusCode, String);

//     async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
//         let (parts, body) = req.into_parts();
//         let req = Request::from_parts(parts, body);

//         let json = Json::<LogoutRequest>
//             ::from_request(req.clone(), state.as_ref()).await
//             .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
//         let jar = CookieJar::from_request(req, state.as_ref()).await.map_err(|e| (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             e.to_string(),
//         ))?;

//         Ok(LogoutRequest { json, jar })
//     }
// }
