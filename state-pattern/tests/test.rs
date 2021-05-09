use state_pattern::Post;

#[test]
fn blog_post_test() {
    let mut post = Post::new("Hello");
    assert_eq!(post.content(), "");

    post.pending_review();
    assert_eq!(post.content(), "");

    post.approve();
    assert_eq!(post.content(), "Hello");
}
