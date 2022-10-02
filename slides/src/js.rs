pub(crate) fn focus_slideshow() {
    js! {
        setTimeout(
            () => {
                document.getElementById("slideshow").focus(); console.log("Focused slideshow")
            }
            , 200
        );
    }
}

use stdweb::js;
