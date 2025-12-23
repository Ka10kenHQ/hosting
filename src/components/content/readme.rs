use yew::{Html, function_component, html};

#[function_component(Readme)]
pub fn readme() -> Html {
    html! {
        <div class="readme-container">
            <h1>{"Mate Kopaliani"}</h1>
            <p class="subtitle">{"Software Engineer | CS Student"}</p>

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
                    <h3>{"Bachelor's in Computer Science"}</h3>
                    <p>{"Minor in Mathematics"}</p>
                    <p>
                        {"Current Cummulative Gpa"}
                        <br/>
                        <strong>{"GPA: 3.5"}</strong>
                    </p>
                </div>

                <div class="info-card">
                    <h2>{" Tech Stack"}</h2>
                    <div class="tech-badges">
                        <span class="badge">{"Go"}</span>
                        <span class="badge">{"C#"}</span>
                        <span class="badge">{"Rust"}</span>
                        <span class="badge">{"Python"}</span>
                        <span class="badge">{"Java"}</span>
                        <span class="badge">{"Lua"}</span>
                    </div>
                </div>

                <div class="info-card work-experience-card">
                    <h2>{" Work Experience"}</h2>
                    <div class="experience-item">
                        <h3>{"Back-End Software Engineer"}</h3>
                        <p>
                            <a href="https://www.cvcertified.com/" target="_blank">
                                {"CVCertified"}
                            </a>
                            {" (Current) - Developing scalable back-end systems and RESTful APIs \
                             for enterprise applications. Working with microservices architecture, \
                             database optimization, implementing robust authentication and authorization \
                             systems, and ensuring high performance and reliability."}
                        </p>
                    </div>
                    <div class="experience-item">
                        <h3>{"Back-End Developer Intern"}</h3>
                        <p>
                            <a href="https://api24.ge/home" target="_blank">
                                {"API24"}
                            </a>
                            {" - Built and maintained server-side applications, designed database schemas, \
                             and integrated third-party services. Gained experience in API design patterns \
                             and cloud deployment practices."}
                        </p>
                    </div>
                    <div class="experience-item">
                        <h3>{"Student Tutor of Linear Algebra"}</h3>
                        <p>
                            {"KIU University - Tutored undergraduate students in Linear Algebra, \
                             helping them understand complex mathematical concepts including vector spaces, \
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
