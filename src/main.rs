use yew::prelude::*;
use yew::html;
use yew_octicons::{Icon, IconKind};

pub enum Msg {
    Goto(Song),
    Next,
    Prev,
}

#[derive(Clone, PartialEq)]
struct Song {
    title: String,
    author: String,
    album: String,
    url: String,
    img: String,
}

#[derive(Clone, PartialEq)]
struct PlayList {
    name: String,
    list: Vec<Song>,
    img: String,
}

#[derive(Properties, PartialEq)]
struct SongProp {
    song: Song,
}

#[derive(Properties, PartialEq)]
struct PlayListProp {
    playlist: PlayList,
    onclick: Callback<Song>,
}

#[derive(Clone, PartialEq)]
enum Page {
    Liked(PlayList),
    Custom(PlayList),
    PlayLists(Vec<PlayList>),
}

#[derive(Properties, PartialEq)]
struct PageProp {
    page: Page,
    onclick: Callback<Song>,
}

#[function_component(SongList)]
fn song_list(PlayListProp {playlist, onclick}: &PlayListProp) -> Html{
    let list_view = playlist.list.iter().map(|song| {
        let on_select = {
            let onclick = onclick.clone();
            let song = song.clone();
            Callback::from(move |_| {
                onclick.emit(song.clone())
            })
        };

        html! {
            <tr onclick={on_select} class={"hover"}>
                <td class={"flex items-center gap-3"}>
                    <div class={"avatar"}>
                        <div class={"rounded w-12 h-12"}>
                            <img src={"https://wpimg.pixelied.com/blog/wp-content/uploads/2021/06/15134429/Spotify-Good-Contrast-Examples-2-480x476.jpg"} />
                        </div>
                    </div>
                    <div>
                        <p class={"font-bold"}>{song.title.clone()}</p>
                        <p>{song.author.clone()}</p>
                    </div>
                </td>
                <td>
                    <p>{song.album.clone()}</p>
                </td>
            </tr>
        }
    }).collect::<Html>();

    html! {
        <div class="flex">
            <table role={"role"} class={"table table-pin-rows"}>
                <thead>
                    <tr>
                        <th>{"Title"}</th>
                        <th>{"Album"}</th>
                    </tr>
                </thead>
                <tbody>
                    {list_view}
                </tbody>
            </table>
        </div>
    }
}

#[function_component(MainView)]
fn main_view(PageProp {page, onclick}: &PageProp) -> Html {

    let html = match page {
        Page::Liked(x) => {html! {
            <div>
                <p>{x.name.clone()}</p>
                <SongList playlist={x.clone()} onclick={onclick} />
            </div>
        }},
        Page::Custom(x) => {html!{
            <div>
                <p>{format!("{}, but Custom!", x.name.clone())}</p>
                <SongList playlist={x.clone()} onclick={onclick} />
            </div>
        }},
        Page::PlayLists(vec) => {html!{
            <div>
                <p>{"NOTHING TO SEE BITCH"}</p>
            </div>
        }},
    };

    html! {
        <div class="flex-1 overflow-auto">
            {html}
        </div>
    }
} 

#[function_component(Footer)]
fn footer(SongProp {song}: &SongProp) -> Html {



    html! {
        <footer class="footer flex  text-neutral-content">
        // <footer class={"btm-nav footer sp-3 bg-neutral text-neutral-content"}>
            <p>{song.title.clone()}</p>
            <progress class={"progress w-full h-1"}/>
        </footer>
    }
}

#[function_component(Menu)]
fn menu() -> Html {

    html! {
        <ul class="menu bg-base-200 rounded-box flex-none">
            <li class="menu-title">{"Menu"}</li>
            <li><a class={"font-medium"}>
                { Icon::new(IconKind::Home) }
                {"Home"}
                </a></li>
            <li><a>
                { Icon::new(IconKind::Search) }
                {"Search"}
                </a></li>
            <li><a>
                { Icon::new(IconKind::Gear) }
                {"Settings"}</a></li>
            <li><a>
                { Icon::new(IconKind::ScreenFull) }
                {"Full Screen"}
            </a></li>
            <li><a>

            </a></li>
        </ul>
    }
}

#[function_component(App)]
fn app() -> Html {

    let list = vec!{
        Song {
            title: "Stressed Out".to_string(),
            author: "Twenty One Pilots".to_string(),
            album: "Blurryface".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Night Witches".to_string(),
            author: "Sabaton".to_string(),
            album: "Heroes".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Bones".to_string(),
            author: "Imagine Dragons".to_string(),
            album: "Mercury".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }, Song {
            title: "Why Am I the One".to_string(),
            author: "Fun".to_string(),
            album: "Some Nigths".to_string(),
            url: "...".to_string(),
            img: "...".to_string(),
        }
    };

    let playlist = PlayList {
        name: "Liked List".to_string(),
        list: list.clone(),
        img: "test".to_string(),
    };

    let playing: UseStateHandle<Option<Song>> = use_state(|| Some(list[0].clone()));
    let select = {
        let playing = playing.clone();
        Callback::from(move |song| {
            playing.set(Some(song))
        })
    };

    let page = Page::Liked(playlist);

    let details = playing.as_ref().unwrap().clone();

    html! {
        <div class="flex flex-col h-screen">
            <div class="flex-1 flex overflow-y-hidden">
                <Menu/>
                <MainView page={page} onclick={select} />
                // <SongList list={list} onclick={select} />
            </div>
            <Footer song={details}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}