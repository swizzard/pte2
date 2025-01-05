use crate::from_toml::{Exercise, Exercises};
use html::{metadata::builders::HeadBuilder, text_content::builders::DivisionBuilder};

fn build_head(head: &mut HeadBuilder) -> &mut HeadBuilder {
    head.meta(|mt| mt.charset("UTF-8"))
        .meta(|mt| {
            mt.name("viewport")
                .content("width=device-width, initial-scale=1.0")
        })
        .meta(|mt| mt.http_equiv("X-UA-Compatible").content("ie=edge"))
        .title(|ti| ti.text("pte"))
        .link(|li| li.href("style.css").rel("stylesheet"))
}

fn build_container(db: &mut DivisionBuilder, exercises: Vec<Exercise>) -> &mut DivisionBuilder {
    db.class("container")
        .heading_1(|h1| h1.text("exercises"))
        .division(|dbb| {
            dbb.class("resetAll").button(|bb| {
                bb.id("resetAll")
                    .type_("button")
                    .class("resetAll")
                    .text("reset all")
            })
        })
        .division(|dbb| build_exercises(dbb, exercises).script(|sb| sb.src("index.js")))
}

fn build_ex(db: &mut DivisionBuilder, Exercise { label, reps }: Exercise) -> &mut DivisionBuilder {
    db.class("ex")
        .heading_2(|h2| h2.text(label))
        .division(|dbb| {
            dbb.button(|bb| bb.type_("button").class("resetButton").text("reset"));
            for _ in 0..reps {
                dbb.button(|bb| bb.type_("button").class("repButton").text("✔️"));
            }
            dbb
        })
}

fn build_exercises(db: &mut DivisionBuilder, exercises: Vec<Exercise>) -> &mut DivisionBuilder {
    db.class("content");
    for ex in exercises.into_iter() {
        db.division(|dbb| build_ex(dbb, ex));
    }
    db
}

pub fn write_html(Exercises { exercises }: Exercises) -> String {
    let ht = html::root::Html::builder()
        .lang("en")
        .head(|h| build_head(h))
        .body(|bb| bb.division(|db| build_container(db, exercises)))
        .build();
    ht.to_string()
}
