use leptos::*;
use todos::models::TodoItem;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    let (todo_input, set_todo_input) = create_signal(String::new());
    let (items, set_items) = create_signal(vec![
        TodoItem::new("Learn Leptos".to_string()),
        TodoItem::new("Build a Todo App".to_string()),
        TodoItem::new("Profit!".to_string()),
    ]);
    let to_complete = move || items().into_iter().filter(|item: &TodoItem| !item.completed.get());
    let completed = move || items().into_iter().filter(|item: &TodoItem| item.completed.get());


    mount_to_body(move || view! {
        <div class="container">
            <h1 class="text-center mt-4">{ "Todos" }</h1>
            <p/>

            <div class="container">
                <div class="card rounded-4 shadow mb-3">
                    <div class="card-body">
                        <div class="input-group mb-3">
                            <input type="text" class="form-control" placeholder="Todo Description" prop:value=todo_input on:input=move |ev| {
                                set_todo_input(event_target_value(&ev));
                            }/>
                            <button class="btn btn-primary input-group-text fw-bold" on:click=move |_| {
                                if !todo_input().is_empty() {
                                    set_items.update(|items| items.push(TodoItem::new(todo_input())));
                                    set_todo_input(String::new());
                                }
                            }>+</button>
                        </div>
                    </div>
                </div>

                <For each=to_complete key=move |item| item.id children=move |item| {
                    view! {
                        <div class="card rounded-4 shadow-sm mt-3">
                            <div class="card-body">
                                <input class="form-check-input" type="checkbox" id=item.id on:input=move |_| {
                                    item.completed.update(|completed| *completed = !*completed);
                                }>
                                </input>
                                <label class="form-check-label ps-2 fs-6" for=item.id>
                                    { item.text }
                                </label>
                                <button class="btn btn-danger float-end" on:click=move |_| {
                                    set_items.update(|items| items.retain(|i| i.id != item.id));
                                }>{"X"}</button>
                            </div>
                        </div>
                    }
                } />

                <p/>

                <For each=completed key=move |item| item.id children=move |item| {
                    view! {
                        <div class="card rounded-4 shadow-sm mt-3">
                            <div class="card-body">
                                <input class="form-check-input" disabled="true" type="checkbox" prop:checked=item.completed>
                                </input>
                                <label class="form-check-label ps-2 fs-6" for=item.id>
                                    { item.text }
                                </label>
                            </div>
                        </div>
                    }
                } />
            </div>
        </div>
    });
}
