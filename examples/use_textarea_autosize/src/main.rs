use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_textarea_autosize, UseTextareaAutosizeReturn};

#[component]
fn Demo() -> impl IntoView {
    let textarea = NodeRef::new();

    let UseTextareaAutosizeReturn {
        content,
        set_content,
        ..
    } = use_textarea_autosize(textarea);

    view! {
        <div class="mb-4">Type, the textarea will grow:</div>
        <textarea
            prop:value=content
            on:input=move |evt| set_content.set(event_target_value(&evt))
            node_ref=textarea
            class="resize-none box-border"
            placeholder="What's on your mind?"
        />
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let unmount_handle = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });

    unmount_handle.forget();
}
