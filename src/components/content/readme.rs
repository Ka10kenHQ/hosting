use yew::{Html, function_component, html};

#[function_component(Readme)]
pub fn readme() -> Html {
    html! {
        <div class="readme-container">
            <h1>{"Mate Kopaliani"}</h1>

            <div class="cards-grid">
                <div class="info-card">
                    <h2>{"About Me"}</h2>
                    <p>
                        {"Hi, I'm Mate Kopaliani and welcome to my portfolio page. \
                         If you are interested, you can check out my projects and source code :)"}
                    </p>
                    <p>
                        {"I'm a 20-year-old Computer Science student from Georgia. \
                         Love working with Linux, terminals, back-end development (system design), and AI/ML related topics."}
                    </p>
                </div>


                <div class="info-card">
                    <h2>{" Education"}</h2>
                    <h4>{"Bachelor's in Computer Science with Minor in Mathematics"}</h4>
                    <p>
                        {"I am in my 4th year of CS/Math degree with "}
                        {"Current Cummulative Gpa"}
                        <br/>
                        <strong>{"GPA: 3.5/4.0"}</strong>
                        <br/>
                        {"Average Grade:"}
                        <br/>
                        <strong> {"92/100"}</strong>
                    </p>
                </div>

                <div class="info-card work-experience-card">
                    <h2>{" Work Experience"}</h2>
                    <div class="experience-item">
                        <h3>{"Back-End Software Engineer"}</h3>
                        <p>
                            <a href="https://www.cvcertified.com/" target="_blank">
                                {"Collision Vision"}
                            </a>
                            {" (Current) - developing scalable back-end systems \
                             for enterprise application. working with .NET, azure, python"}
                        </p>
                    </div>
                    <div class="experience-item">
                        <h3>{"Back-End Developer Intern"}</h3>
                        <p>
                            <a href="https://api24.ge/home" target="_blank">
                                {"API24"}
                            </a>
                            {" - Built and maintained back-end primarily focused on document processing with OCR,
                            pattern recognition and translation
                            "}
                        </p>
                    </div>
                    <div class="experience-item">
                        <h3>{"Student Tutor of Linear Algebra"}</h3>
                        <p>
                            {"KIU University - Tutored undergraduate students in Linear Algebra, \
                             helping them understand mathematical concepts including vector spaces, \
                             matrix operations, eigenvalues, and linear transformations."}
                        </p>
                    </div>
                </div>

                <div class="info-card">
                    <h2>{" Interests"}</h2>
                    <ul>
                        <li>{"Distributed systems & microservices"}</li>
                        <li>{"Machine learning & artificial intelligence"}</li>
                        <li>{"Systems programming & optimization"}</li>
                        <li>{"Developer tooling & productivity"}</li>
                    </ul>
                </div>

                <div class="info-card contact-card">
                    <h2>{" Get In Touch"}</h2>
                    <div class="contact-links">
                        <a href="mailto:matekopaliani12@gmail.com" class="contact-link">
                            <span class="icon">{"✉️"}</span>
                            {"Email Me"}
                        </a>
                        <a href="www.linkedin.com/in/mate-kopaliani-8838a7277" target="_blank" class="contact-link">
                            <span class="icon">{"🔗"}</span>
                            {"LinkedIn"}
                        </a>
                        <a href="https://github.com/Ka10ken1" target="_blank" class="contact-link">
                            <span class="icon">{"🐙"}</span>
                            {"GitHub"}
                        </a>
                    </div>
                </div>
            </div>

            <hr/>
            <p class="footer-text">{"Thanks for stopping by! "}</p>
        </div>
    }
}
