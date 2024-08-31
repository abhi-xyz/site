use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    // Because the design has a width of 960px, we will also need to insert a wrapper into each section. This is done using a container div with a class of 'wrapper':
    html! {
    <>
    <header>
    <div class="wrapper">
    <h1>{ "ABHINANDH S" }<span class="color">{ "." }</span></h1>
    <nav>
    <ul>
    <li><a href="#">{ "Home" }</a></li>
    <li><a href="#">{ "Portfolio" }</a></li>
    <li><a href="#">{ "Contact" }</a></li>
    <li><a href="#">{ "Blog" }</a></li>
    </ul>
    </nav>
    </div>
    </header>
    /*
    The hero banner image is a simple div with the headline text in a <h2> and button link.
    */

    <div id="hero-image">
    <div class="wrapper">
    <h2><strong>{ "A Minimal, clean" }</strong><br/>
    { "layout for web design." }</h2>
    <a href="#" class="button-1">{ "Get Started" }</a>
    </div>
    </div>

    // Photo by <a href="https://unsplash.com/@crismiron?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash">Miron Cristina</a> on <a href="https://unsplash.com/photos/gray-and-white-cat-showing-tongue-HW_6USwudbo?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash">Unsplash</a>

    /*
    The 3 feature sections are each a list item in an unordered list, using CSS we will float the list items left to display them horizontally. To prevent other elements being affected by the floating we will also need a div with the class 'clear' and in the CSS later the class '.clear' will have the attribute 'clear:both'.
    */

    <div id="features">
    <div class="wrapper">
    <ul>
    <li class="feature-1">
    <h4>{ "Easy to Edit" }</h4>
    <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam mollis turpis ac libero interdum, id fringilla purus feugiat. Etiam malesuada mattis nibh at bibendum." }</p>
    </li>
    <li class="feature-2">
    <h4>{ "Layered PSD" }</h4>
    <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam mollis turpis ac libero interdum, id fringilla purus feugiat. Etiam malesuada mattis nibh at bibendum." }</p>
    </li>
    <li class="feature-3">
    <h4>{ "Ready to Go" }</h4>
    <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam mollis turpis ac libero interdum, id fringilla purus feugiat. Etiam malesuada mattis nibh at bibendum." }</p>
    </li>
    <div class="clear"></div>
    </ul>
    </div>
    </div>

    /*
    The primary content section is very simple and uses an <img> tag for the video placeholder image.
    */

    //FIX: image not showing up.
    <div id="primary-content">
    <div class="wrapper">
    <article>
    <h3>{ "Featured Content" }</h3>
    <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec euismod leo a nibh dignissim tincidunt. Nam ultricies odio ac neque suscipit volutpat. Ut dictum adipiscing felis sed malesuada. Integer porta sem nec nibh hendrerit imperdiet." }</p>
    <a href="#"><img src="assets/cat.jpg" alt="video placeholder" /></a>
    </article>
    </div>
    </div>
    /*
    Similarly to the features section, we are floating each <article> left to display them horizontally. You will probably notice in this section that we have used some inline styles. I would not normally recommend this, but in this instance the background image for the articles would be classed as content rather than styling so it belongs in the HTML not CSS.
    */


    <div id="secondary-content">
    <div class="wrapper">
    <article style="background-image: url('assets/cat.jpg');">
    <div class="overlay">
    <h4>{ "Secondary Content" }</h4>
    <p><small>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec euismod leo a nibh dignissim tincidunt nam." }</small></p>
    <a href="#" class="more-link">{ "View more" }</a>
    </div>
    </article>
    <article style="background-image: url('assets/cat.jpg');">
    <div class="overlay">
    <h4>{ "Secondary Content" }</h4>
    <p><small>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec euismod leo a nibh dignissim tincidunt nam." }</small></p>
    <a href="#" class="more-link">{ "View more" }</a>
    </div>
    </article>
    <div class="clear"></div>
    </div>
    </div>
    /*
    Another straightforward section featuring just a heading 3, paragraph and button link.
    */
    <div id="cta">
    <div class="wrapper">
    <h3>{ "Heard Enough?" }</h3>
    <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec euismod leo a nibh dignissim tincidunt. Nam ultricies odio ac neque suscipit volutpat. Ut dictum adipiscing felis sed malesuada. Integer porta sem nec nibh hendrerit imperdiet." }</p>
    <a href="#" class="button-2">{ "Get Started" }</a>
    </div>
    </div>
    /*
    The footer is divided into two columns, the 'footer-info' column and the 'footer-links' column. The 'footer-links' column is then sub-divided into 3 columns using unordered lists.
    */

    <footer>
    <div class="wrapper">
    <div id="footer-info">
    <p>{ "Copyright 2014 CompanyName. All rights reserved." }</p>
    <p>
    <a href="#">{ "Terms of Service" }</a>
    <a href="#">{ "Privacy" }</a>
    </p>
    </div>
    <div id="footer-links">
    <ul>
    <li><h5>{ "Company" }</h5></li>
    <li><a href="#">{ "About Us" }</a></li>
    <li><a href="#">{ "Meet The Team" }</a></li>
    <li><a href="#">{ "What We Do" }</a></li>
    <li><a href="#">{ "Careers" }</a></li>
    </ul>
    <ul>
    <li><h5>{ "Services" }</h5></li>
    <li><a href="#">{ "Consulting" }</a></li>
    <li><a href="#">{ "Support" }</a></li>
    <li><a href="#">{ "Training" }</a></li>
    <li><a href="#">{ "Maintenance" }</a></li>
    </ul>
    <ul>
    <li><h5>{ "Contact" }</h5></li>
    <li><a href="#">{ "Contact Us" }</a></li>
    <li><a href="#">{ "Locations" }</a></li>
    <li><a href="#">{ "FAQ" }</a></li>
    <li><a href="#">{ "Feedback" }</a></li>
    </ul>
    </div>
    <div class="clear"></div>
    </div>
    </footer>
    /*
    The next step is to link to some css files that we will be using. The first one is a css reset from Meyerweb that can be downloaded here: CSS Reset. Once it has been downloaded place it into the css folder and include this line of code in the <head> section of our HTML:

    Then we want to include a separate css file for our own custom styles. Create a new blank document with your text editor and save it as style.css, place it in the css folder and then include this line of code in the <head> section of the HTML (directly below the css reset line):
    */

    </>
    }
}
