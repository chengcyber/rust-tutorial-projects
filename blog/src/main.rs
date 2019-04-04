/**
 * state pattern
 * 1. A blog post starts as an empty draft.
 * 2. When the draft is done, a review of the post is requested.
 * 3. When the post is approved, it gets published.
 * 4. Only published blog posts return content to print, so unapproved posts can's accientally be published.
 * 5. A reject method that cahgnes the post's state from PendingReview back to Draft.
 * 6. Require two calls to approve before the state can be changed to Published
 * 7. Allow users to add text content only when a post is in the Draft state.
 */
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // only allow add text in draft state
    post.add_text("...");
    assert_eq!("", post.content());

    // back to draft
    post.reject();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.request_review();

    // approve twice
    post.approve();
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
