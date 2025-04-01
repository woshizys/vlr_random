use crate::generator::generate;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;
use std::str::FromStr;

pub async fn serve() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(show_form))
        .route("/", post(show_form))
        .route("/submit", post(submit));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn show_form() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>随机英雄分配器</title>
        </head>
        <body>
            <form method="post" action="/submit">
            <h1>请输入至多 5 个 ID（空ID视为该玩家不存在）</h1>
                <input type="text" name="id1" placeholder="ID 1"><br>
                <input type="text" name="id2" placeholder="ID 2"><br>
                <input type="text" name="id3" placeholder="ID 3"><br>
                <input type="text" name="id4" placeholder="ID 4"><br>
                <input type="text" name="id5" placeholder="ID 5"><br>
            <h1>请选择阵容限制：（不应大于玩家数，不限阵容可以全选0，小于玩家数的部分将会随机分配）</h1>
                决斗数量：
                <label>
                    <input type="radio" name="duelist" value="0" checked> 0
                </label>
                <label>
                    <input type="radio" name="duelist" value="1"> 1
                </label>
                <label>
                    <input type="radio" name="duelist" value="2"> 2
                </label>
                <label>
                    <input type="radio" name="duelist" value="3"> 3
                </label>
                <label>
                    <input type="radio" name="duelist" value="4"> 4
                </label>
                <label>
                    <input type="radio" name="duelist" value="5"> 5
                </label>
                <br>
                哨位数量：
                <label>
                    <input type="radio" name="sentinel" value="0" checked> 0
                </label>
                <label>
                    <input type="radio" name="sentinel" value="1"> 1
                </label>
                <label>
                    <input type="radio" name="sentinel" value="2"> 2
                </label>
                <label>
                    <input type="radio" name="sentinel" value="3"> 3
                </label>
                <label>
                    <input type="radio" name="sentinel" value="4"> 4
                </label>
                <label>
                    <input type="radio" name="sentinel" value="5"> 5
                </label>
                <br>
                控场数量：
                <label>
                    <input type="radio" name="controller" value="0" checked> 0
                </label>
                <label>
                    <input type="radio" name="controller" value="1"> 1
                </label>
                <label>
                    <input type="radio" name="controller" value="2"> 2
                </label>
                <label>
                    <input type="radio" name="controller" value="3"> 3
                </label>
                <label>
                    <input type="radio" name="controller" value="4"> 4
                </label>
                <label>
                    <input type="radio" name="controller" value="5"> 5
                </label>
                <br>
                先锋数量：
                <label>
                    <input type="radio" name="initiator" value="0" checked> 0
                </label>
                <label>
                    <input type="radio" name="initiator" value="1"> 1
                </label>
                <label>
                    <input type="radio" name="initiator" value="2"> 2
                </label>
                <label>
                    <input type="radio" name="initiator" value="3"> 3
                </label>
                <label>
                    <input type="radio" name="initiator" value="4"> 4
                </label>
                <label>
                    <input type="radio" name="initiator" value="5"> 5
                </label>
                <br>
                <button type="submit">提交</button>
            </form>
        </body>
        </html>
    "#)
}

async fn submit(Form(form): Form<InputForm>) -> Html<String> {
    let duelist = u32::from_str(&form.duelist).unwrap();
    let sentinel = u32::from_str(&form.sentinel).unwrap();
    let controller = u32::from_str(&form.controller).unwrap();
    let initiator = u32::from_str(&form.initiator).unwrap();
    let mut players = 0;
    let id1 = form.id1;
    let id2 = form.id2;
    let id3 = form.id3;
    let id4 = form.id4;
    let id5 = form.id5;
    if !id1.is_empty() {
        players += 1;
    }
    if !id2.is_empty() {
        players += 1;
    }
    if !id3.is_empty() {
        players += 1;
    }
    if !id4.is_empty() {
        players += 1;
    }
    if !id5.is_empty() {
        players += 1;
    }
    if duelist + sentinel + controller + initiator > players {
        return Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>随机英雄分配器</title>
        </head>
        <body>
            <h1>阵容所需玩家数大于当前玩家数！</h1>
            <form method="post" action="/">
                <button type="return">返回</button>
            </form>
        </body>
        </html>
    "#.parse().unwrap());
    }
    let mut team = generate(players, duelist, sentinel, controller, initiator);
    let mut res = String::new();
    if !id1.is_empty() {
        res.push_str(&format!("<h1>{}: {}</h1>", id1, team.pop().unwrap()));
    }
    if !id2.is_empty() {
        res.push_str(&format!("<h1>{}: {}</h1>", id2, team.pop().unwrap()));
    }
    if !id3.is_empty() {
        res.push_str(&format!("<h1>{}: {}</h1>", id3, team.pop().unwrap()));
    }
    if !id4.is_empty() {
        res.push_str(&format!("<h1>{}: {}</h1>", id4, team.pop().unwrap()));
    }
    if !id5.is_empty() {
        res.push_str(&format!("<h1>{}: {}</h1>", id5, team.pop().unwrap()));
    }
    Html(format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>随机英雄分配器</title>
        </head>
        <body>
            <h1>你们的阵容是：</h1>
            <p>{}</p>
            <form method="post" action="/">
                <button type="return">返回</button>
            </form>
        </body>
        </html>
    "#, res))
}

#[derive(Deserialize)]
struct InputForm {
    id1: String,
    id2: String,
    id3: String,
    id4: String,
    id5: String,
    duelist: String,
    sentinel: String,
    controller: String,
    initiator: String,
}
