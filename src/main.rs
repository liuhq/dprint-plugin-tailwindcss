use std::path::Path;

use dprint_plugin_tailwindcss::{FormatTextOptions, configuration, format_text};

fn main() -> anyhow::Result<()> {
    let source_text = r#"import React from 'react';

/**
 * A simple counter component
 */
export const Counter: React.FC = () => {
    const [count, setCount] = React.useState(0);
    const tempStr = `this is a ${count}, don't split by \n, plz`;

    return (
        <div class='p-2 m-3 w-full hover:bg-red *:text-sm'>
            <p class='text-blue'>Count: {count}</p>
            <div class='m-auto grid min-h-screen select-none grid-rows-[auto_1fr_auto] place-items-center overflow-x-hidden bg-ctp-base text-ctp-text scrollbar-thin scrollbar-thumb-ctp-surface0 hover:scrollbar-thumb-ctp-surface2 md:w-[calc(768px+100vw-100%)] md:pl-[calc(100vw-100%)]'>DIV</div>
            <button
                class='m-auto grid min-h-screen select-none grid-rows-[auto_1fr_auto] place-items-center overflow-x-hidden bg-ctp-base text-ctp-text scrollbar-thin scrollbar-thumb-ctp-surface0 hover:scrollbar-thumb-ctp-surface2 md:w-[calc(768px+100vw-100%)] md:pl-[calc(100vw-100%)]'
                onClick={() => setCount(count + 1)}
            >
                Increment
            </button>
            <button class={'w-full h-full place-content-center-safe *:[&_img,&_svg]:w-1/4 *:[&_img,&_svg]:m-auto'} onClick={() => setCount(count - 1)}>Decrement</button>
            <button class={'m-auto grid min-h-screen select-none grid-rows-[auto_1fr_auto] place-items-center overflow-x-hidden bg-ctp-base text-ctp-text scrollbar-thin scrollbar-thumb-ctp-surface0 hover:scrollbar-thumb-ctp-surface2 md:w-[calc(768px+100vw-100%)] md:pl-[calc(100vw-100%)]'} onClick={() => setCount(count - 1)}>Decrement</button>
            <article class="prose max-w-none select-text pt-4 text-ctp-text dark:prose-invert selection:bg-ctp-lavender selection:text-ctp-crust prose-headings:text-ctp-text prose-h2:mt-14 prose-h2:text-ctp-lavender prose-h3:mb-5 prose-h3:mt-12 prose-h3:text-ctp-subtext1 prose-h4:mb-4 prose-h4:mt-10 prose-h4:w-fit prose-h4:border-l-8 prose-h4:border-ctp-lavender prose-h4:bg-ctp-surface0 prose-h4:px-3 prose-h4:py-1 prose-h4:text-ctp-subtext0 prose-a:text-ctp-lavender prose-a:no-underline prose-blockquote:border-ctp-surface0 prose-blockquote:not-italic prose-blockquote:text-ctp-overlay1 prose-strong:font-bold prose-strong:text-ctp-teal prose-em:italic prose-em:text-ctp-sky prose-kbd:select-none prose-kbd:border prose-kbd:border-b-4 prose-kbd:border-ctp-surface2 prose-kbd:text-ctp-subtext1 prose-kbd:shadow-none prose-pre:rounded prose-pre:border-4 prose-pre:border-ctp-crust prose-pre:bg-ctp-crust prose-pre:text-ctp-text prose-pre:scrollbar-thin prose-pre:scrollbar-thumb-ctp-surface0 prose-th:text-ctp-lavender prose-img:mx-auto prose-img:rounded prose-img:shadow-md prose-img:shadow-ctp-crust prose-hr:border-ctp-surface2 hover:prose-a:underline hover:prose-kbd:border-ctp-lavender hover:prose-kbd:text-ctp-lavender hover:prose-pre:scrollbar-thumb-ctp-surface2 prose-inline-code:break-words prose-inline-code:rounded prose-inline-code:bg-ctp-surface0 prose-inline-code:px-2 prose-inline-code:py-1 prose-inline-code:text-ctp-subtext1 prose-inline-code:before:content-[''] prose-inline-code:after:content-[''] md:prose-hr:mx-12">
                {markdown}
            </article>
        </div>
    )
}"#;

    match format_text(FormatTextOptions {
        path: &Path::new("Counter.tsx"),
        extension: None,
        text: source_text.to_string(),
        config: &configuration::Configuration::default()
            .with_indent_to_quote(true)
            .with_indent_width(4)
            .with_line_width(120)
            .with_allow_line_overflow(false)
            .with_line_width_includes_indent(true),
    })? {
        Some(text) => println!("{text}"),
        None => todo!(),
    }

    Ok(())
}
