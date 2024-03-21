use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct UsersListProps {
    pub users: Vec<String>,
    pub username: String,
}

#[function_component(UsersList)]
pub fn get_users_list(props: &UsersListProps) -> Html {
    let UsersListProps { users, username } = props;
    html! {
        <div class="users-list-wrapper">
            <h3>{"Active Users"}</h3>
            <ul class="users-list">
                <li class="active-user">{username}<span class="active-user-you">{"You"}</span></li>
                {
                    users.iter().filter(|u| u.as_str() != username).map(|user| {
                        html! {
                            <li class="active-user">{user}</li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}