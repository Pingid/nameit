use leptos::*;

/// Renders the npm package availability
#[component]
pub fn Badge<L: Fn() -> bool + 'static, A: Fn() -> Option<bool> + 'static>(
    icon: String,
    loading: L,
    available: A,
    #[prop(default = "".to_string())] label: String,
) -> impl IntoView {
    let status = create_memo(move |_| {
        match loading() {
        true => "icon-[svg-spinners--270-ring-with-bg] w-5 h-5 relative top-[2px] left-[2px] text-black/70",
        false => match available() {
            Some(true) => "icon-[heroicons--check-circle] w-6 h-6 text-green-800",
            Some(false) => "icon-[heroicons--x-circle] w-6 h-6 text-red-800",
            None => "icon-[heroicons--x-circle] w-6 h-6 text-black/10"
        },
    }
    });

    view! {
        <div class="rounded-full px-2 py-1 border flex items-center gap-1">
            <p class="text-sm flex items-center gap-1">
                <span class=format!("w-6 h-6 relative top-[1px] {}", icon)></span>
                <span>{label}</span>
            </p>
            <div class="w-6 h-6 flex">
                <span class=status></span>
            </div>
        </div>
    }
}
