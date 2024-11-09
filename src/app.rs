use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptoaster::*;
use leptos_darkmode::*;
use thaw::*;
use web_sys::MouseEvent;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_toaster();
    let dark_mode = Darkmode::init();
    view! {
        <Stylesheet id="leptos" href="/pkg/xsia_leptos_ui_csr.css" />
        <Title text="xSIA Template" />
        <Html lang="en" class=move || if dark_mode.is_dark() { "dark" } else { "" }/>
        <Toaster stacked=true />
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Home /> } />
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let theme = RwSignal::new(Theme::dark());
    let toaster = expect_toaster();
    let placement = RwSignal::new(DrawerPlacement::Right);
    let show_menu = RwSignal::new(true);
    let value = RwSignal::new(String::from("o"));
    let mut dark_mode = expect_context::<Darkmode>();

    let theme_name = Memo::new(move |_| {
        theme.with(|theme| {
            if theme.name == *"light" {
                "Dark".to_string()
            } else {
                "Light".to_string()
            }
        })
    });
    // let (_, write_theme, _) = use_local_storage::<String, FromToStringCodec>("theme");
    let change_theme = {
        let mut mode_dark = dark_mode.clone();
        if theme_name.get_untracked() == "Light" {
            theme.set(Theme::light());
            // write_theme.set("light".to_string());
            mode_dark.set_light();

        } else {
            theme.set(Theme::dark());
            // write_theme.set("dark".to_string());
            mode_dark.set_dark();
        }
    };

    toaster.toast(
        ToastBuilder::new("My toast message goes here.")
            .with_level(ToastLevel::Success) // set the toast level (default is `ToastLevel::Info`)
            .with_dismissable(false) // allow or disallow the toast from being dismissable (default is `true`)
            .with_expiry(Some(3_000)) // expiry in milliseconds (default is `2500`)
            .with_progress(true) // enable or disable the progress bar (default is `true`)
            .with_position(ToastPosition::TopLeft) // set the toast position (default is 'ToastPosition::BottomLeft`)
    );
    view! {
        <ThemeProvider theme>
            <main class="min-h-screen w-full bg-transparent ">
                <LoaderComponent />
                <header class="sticky top-0 bg-glass shadow z-10">
                    <div class="flex items-center justify-between p-4 mx-auto">
                        <div class="inline-flex items-center">
                            <img
                                class="h-10 w-10"
                                src="/img/android-chrome-512x512.png"
                                alt="logo"
                            />
                            <span class="ml-3 text-gray-700 dark:text-gray-300">xSIA</span>
                        </div>
                        <div class="inline-flex items-center">
                            <Button
                                on_click=change_theme
                            >
                                {move || theme_name.get()}
                            </Button>
                            <ButtonMenu on_click=move |_| show_menu.set(true) />
                        </div>
                    </div>
                </header>
                <Drawer show=show_menu placement mount=DrawerMount::None width="384px" title="Menu">
                    <Menu value default_expanded_keys=vec![String::from("area")]>
                        <MenuItem icon=icondata::AiAreaChartOutlined key="area" label="Area Chart">
                            <MenuItem key="target" label="Target" />
                            <MenuItem key="above" label="Above" />
                            <MenuItem key="below" label="Below" />
                        </MenuItem>
                        <MenuItem icon=icondata::AiPieChartOutlined key="pie" label="Pie Chart">
                            <MenuItem key="pie-target" label="Target" />
                            <MenuItem key="pie-above" label="Above" />
                            <MenuItem key="pie-below" label="Below" />
                        </MenuItem>
                        <MenuItem icon=icondata::AiGithubOutlined key="github" label="Github" />
                        <MenuItem icon=icondata::AiChromeOutlined key="chrome" label="Chrome" />
                    </Menu>
                </Drawer>
            </main>
        </ThemeProvider>
    }
}

#[component]
fn LoaderComponent() -> impl IntoView {
    view! {
        <div id="loading-overlay" class="fixed inset-0 z-10 flex items-center justify-center bg-gray-900 bg-opacity-60 hidden">
            <Spinner size=SpinnerSize::Large/>
        </div>
    }
}

#[component]
fn ButtonTheme() -> impl IntoView {
    let mut darkmode = expect_context::<Darkmode>();
    view! {
        <button
            on:click=move |_| darkmode.toggle()
            class="m-1 ms-0 py-3 px-4 inline-flex items-center gap-x-2 text-sm font-semibold rounded-none text-gray-600 dark:text-gray-400 disabled:opacity-50 disabled:pointer-events-none"
        >
            <svg
                width="24"
                height="24"
                class="svg-icon"
                style="width: 1em; height: 1em;vertical-align: middle;fill: currentColor;overflow: hidden;"
                viewBox="0 0 1024 1024"
                version="1.1"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    d="M320 85.333333c-76.373333 49.066667-128 135.68-128 234.666667s51.626667 185.6 129.28 234.666667C190.293333 554.666667 85.333333 449.706667 85.333333 320A234.666667 234.666667 0 0 1 320 85.333333m493.653333 64l61.013334 61.013334L210.346667 874.666667 149.333333 813.653333 813.653333 149.333333m-263.68 103.68L486.826667 213.333333 425.386667 256l17.92-72.533333L384 138.24l74.666667-5.12 24.746666-70.4L512 132.266667l73.813333 1.28-57.6 48.213333 21.76 71.253333m-140.8 154.026667l-49.493333-31.146667-47.786667 33.28 14.506667-56.32-46.506667-35.413333 58.026667-3.84 19.2-55.04 21.76 54.186667 58.026667 1.28-44.8 37.12 17.066666 55.893333M810.666667 576a234.666667 234.666667 0 0 1-234.666667 234.666667c-52.053333 0-100.266667-17.066667-139.093333-45.653334l328.106666-328.106666c28.586667 38.826667 45.653333 87.04 45.653334 139.093333m-187.733334 280.746667l118.186667-49.066667-10.24 142.933333-107.946667-93.866666m184.746667-115.2l49.066667-118.186667 93.866666 108.373333-142.933333 9.813334m49.066667-211.626667l-48.64-118.613333 142.506666 10.24-93.866666 108.373333M410.88 807.68l118.186667 49.066667-107.946667 93.44-10.24-142.506667z"
                    fill=""
                />
            </svg>
        </button>
    }
}

#[component]
fn ButtonMenu(#[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView {
    view! {
        <button
            on:click=on_click
            class="m-1 ms-0 py-3 px-4 inline-flex items-center gap-x-2 text-sm font-semibold rounded-none text-gray-600 dark:text-gray-400 disabled:opacity-50 disabled:pointer-events-none"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2.25"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="lucide lucide-menu"
            >
                <line x1="4" x2="20" y1="12" y2="12" />
                <line x1="4" x2="20" y1="6" y2="6" />
                <line x1="4" x2="20" y1="18" y2="18" />
            </svg>
        </button>
    }
}
