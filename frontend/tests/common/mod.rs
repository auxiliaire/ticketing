#[macro_export]
macro_rules! assert_expected_output {
    ($component:ty, $props:expr, $expected_output:expr) => {
        let component: ::yew::Renderer<$component> =
            ::yew::Renderer::<$component>::with_root_and_props(
                ::gloo_utils::document()
                    .get_element_by_id("output")
                    .unwrap(),
                $props,
            );

        component.render();

        let output = ::gloo_utils::document()
            .get_element_by_id("output")
            .unwrap();

        assert_eq!(output.inner_html(), $expected_output);
    };
}
