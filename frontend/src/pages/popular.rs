use crate::components::{PlaceholderCardArray, FerrisError, PopularPreviewCard};
use crate::contexts::ServerCtx;
use invidious::{Popular, TrendingCategory::*};
use leptos::*;

#[component]
pub fn PopularSection() -> impl IntoView {
    let server = expect_context::<ServerCtx>().0 .0;

    let popular_videos = create_resource(
        move || server.get(),
        |server| async move { Popular::fetch_popular(&server).await },
    );

    view! {
        <div class="w-full flex justify-center mt-4">
            <div class="w-[90%] flex flex-col gap-y-8">
                <h1 class="pl-4 font-semibold text-2xl">{"Popular"}</h1>
                <Suspense fallback=move || {
                    view! { <PlaceholderCardArray/> }
                }>
                    {move || {
                        popular_videos
                            .get()
                            .map(|popular_videos_res| {
                                match popular_videos_res {
                                    Ok(popular) => {
                                        view! { <PopularVideos popular=popular/> }.into_view()
                                    }
                                    Err(err) => view! { <FerrisError error=err/> },
                                }
                            })
                    }}

                </Suspense>
            </div>
        </div>
    }
}

#[component]
pub fn PopularVideos(popular: Popular) -> impl IntoView {
    let popular_videos_view = popular.items.into_iter().map(|video| view! { <PopularPreviewCard video=video/> }).collect_view();

    view! {
        <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-4rem-128px)] pb-12 overflow-y-auto scroll-smooth">
            {popular_videos_view}
        </div>
    }
}























