use dioxus::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/project/:name")]
    ProjectDetail { name: String },
}

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

// --- GLOBAL FRAME (APPLE MINIMAL WHITE) ---
#[component]
fn Navbar() -> Element {
    rsx! {
        // This includes the CSS asset file dynamically into the application header
        document::Link { rel: "stylesheet", href: asset!("/assets/styles.css") }

        div { class: "min-h-screen bg-white text-neutral-900 font-sans selection:bg-neutral-100 selection:text-neutral-900 antialiased flex flex-col justify-between",
            div {
                nav { class: "max-w-6xl mx-auto px-6 py-5 flex justify-between items-center border-b border-neutral-100",
                    Link { to: Route::Home {}, class: "text-lg font-bold tracking-tight hover:text-neutral-500 transition-colors duration-200",
                        "Alex Vassiliev"
                    }
                    div { class: "flex gap-8 text-xs uppercase tracking-widest font-semibold text-neutral-500",
                        Link { to: Route::Home {}, class: "hover:text-neutral-900 transition-colors", "Home" }
                        a { href: "#experience", class: "hover:text-neutral-900 transition-colors", "Experience" }
                        a { href: "#education", class: "hover:text-neutral-900 transition-colors", "Credentials" }
                    }
                }
                Outlet::<Route> {}
            }

            // --- FOOTER LAYER ---
            footer { class: "border-t border-neutral-100 bg-white mt-20",
                div { class: "max-w-6xl mx-auto px-6 py-10 flex flex-col sm:flex-row justify-between items-center gap-4 text-xs font-mono text-neutral-400",
                    p { class: "flex items-center gap-1.5",
                        span { "Compiled with" }
                        a {
                            href: "https://rust-lang.org/",
                            target: "_blank",
                            class: "inline-flex items-center gap-1 underline hover:text-neutral-900 transition-colors duration-200",
                            img {
                                src: "https://raw.githubusercontent.com/graydon/rust-www/gh-pages/logos/rust-logo-256x256.png",
                                alt: "Rust Logo",
                                class: "w-8 h-8 inline-block object-contain"
                            }
                            "Rust"
                        }
                        span { ". © 2026 Alex Vassiliev. All rights reserved." }
                    }
                    div { class: "flex gap-6 font-semibold uppercase tracking-widest",
                        a { 
                            href: "https://github.com", 
                            target: "_blank", 
                            class: "hover:text-neutral-900 transition-colors duration-200", 
                            "GitHub" 
                        }
                        a { 
                            href: "https://linkedin.com", 
                            target: "_blank", 
                            class: "hover:text-neutral-900 transition-colors duration-200", 
                            "LinkedIn" 
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct JobData {
    id: usize,
    img_src: &'static str,
    alt: &'static str,
    title: &'static str,
    company: &'static str,
    period: &'static str,
    description: &'static str,
    bullets: &'static [&'static str],
}

const JOBS: &[JobData] = &[
    JobData {
        id: 1,
        img_src: "/images/setpoint.png",
        alt: "IT Infrastructure Management",
        title: "IT Manager/System Admin",
        company: "Setpoint Systems Corporation — Littleton, CO https://setpointsystems.com/",
        period: "July 2024 – Present",
        description: "Architect, secure, and administer enterprise hybrid infrastructure across modern/legacy Windows Server environments (2008R2–2022), Linux (Ubuntu LTS), and BAS Delta Controls EnteliWeb server platforms. Oversee full Microsoft 365 tenant administration, enterprise identity management, and multi-cloud environments (AWS/Azure) while leveraging PowerShell, Bash, Python and Ansible to drive infrastructure automation. Engineered custom, in-house applications to support and streamline business operations, managed the company's IT ticketing system from start to finish to keep support running smoothly, and provided direct, advanced remote technical support to keep all field employees connected and working. Visited onsite customers for on-prem troubleshooting and service.",
        bullets: &[
            "Administered, maintained, and set up Windows Server environments (2008R2–2022), Linux Ubuntu LTS Servers, BAS Delta Controls EnteliWeb Server, Synology, including Active Directory, Group Policy, DNS, DHCP, NPS, Radius, print servers including Universal Print. Setup Somain Controllers and Entra Hybrid environment with NIST 800-171 compliance",
            "Led full post-cybersecurity recovery, rebuilt network infrastructure and hardened systems. Deployed and maintained VPN with SAML/SSO authentication on Fortinet Fortigate. Configured and managed Wi-Fi environments, including Guest/SecureCorp. Administered Remote Desktop Services (RDS)/Terminal Servers. Maintained and created print servers including Universal Print.",
            "Windows Hyper-V installed and maintained, including migration and replication. Demonstrated proficiency in Azure VM setup and Azure AD administration.",
            "Demonstrated proficiency developing Rust,Node.js/React.js PostgreSQL apps in Linux environment.",
            "Provided full MS tenant 365 administration: Exchange Online, SharePoint, Defender EDR, InTune, OneDrive, Teams, Entra ID, Intune Deployment including mobile phone management.",
            "Verion Business portal admin including One Talk, phone tree, business Internet,product ordering",
            "Proficiently wrote and deployed PowerShell/Bash and Ansible automation routines.",
            "Demonstrated Synology, Veeam, and Windows backup knowledge. Performed SQL/MySQL & PostgreSQL Server administration. Maintained LAN/WAN networks in the Meraki environment, including SaaS.",
            "Managed AWS environment, including EC2 instance setup, S3 bucket management, IAM user roles and policies, and billing tracking.",
            "Recommended hardware/software for operator workstations. Managed quoting and purchasing of desktops, laptops, servers, and accessories.",
            "Configured and tested systems before deployment; created documentation, including one-line diagrams."
        ]
    },
    JobData {
        id: 2,
        img_src: "/images/intech.png",
        alt: "Full Stack Development Systems",
        title: "Software Engineer",
        company: "Harvard in Tech — Remote https://www.harvardintech.com/ & https://www.factfinderspro.com/",
        period: "Jan 2023 – May 2024",
        description: "Led full-stack web application development with JavaScript, React, Node.js, and RESTful APIs to deliver fast, responsive user interfaces. Built modular front-end components using React Hooks, Context API, and Redux while connecting back-end services to NoSQL databases like MongoDB. Automated testing and deployment processes using CI/CD pipelines with GitHub Actions and Azure DevOps to host applications on Microsoft Azure. Boosted performance and responsiveness using lazy loading, modern CSS frameworks, and extensive unit testing with Jest. Worked closely with design and engineering teams in an Agile Scrum setting using Figma, Jira, and Trello to keep projects moving on schedule.",
        bullets: &[
            "Led full-stack web application development using JavaScript, React.js, CSS3, and RESTful APIs, delivering scalable and responsive user interfaces.",
            "Designed and implemented dynamic front-end components with React Hooks, Context API, and Redux for state management.",
            "Integrated backend services using RESTful APIs and Node.js, ensuring seamless data exchange with NoSQL databases like MongoDB.",
            "Deployed and maintained web applications on Microsoft Azure and GitHub.",
            "Implemented CI/CD pipelines for automated testing and deployment using GitHub Actions and Azure DevOps.",
            "Ensured cross-browser compatibility and mobile responsiveness through flexbox, media queries, and modern CSS frameworks such as Tailwind CSS and Bootstrap.",
            "Applied performance optimization techniques like lazy loading, code splitting, and caching to improve load times and user experience.",
            "Conducted unit testing and integration testing using Jest and React Testing Library to ensure code reliability and coverage.",
            "Followed Agile Scrum methodology with weekly sprints and regular sprint retrospectives.",
            "Collaborated cross-functionally with designers and backend developers using tools like Figma, Trello, and Jira, maintaining high communication standards and clear documentation."
        ]
    },
    JobData {
        id: 3,
        img_src: "/images/qubex.png",
        alt: "Systems Infrastructure Diagnostics",
        title: "IT Support Specialist",
        company: "Qubex Inc. — Denver, CO",
        period: "May 2018 – Jan 2023",
        description: "Managed day-to-day systems administration and technical support across Windows, Linux, and Apple environments. Wrote PowerShell and Bash scripts to automate routine setups, streamline deployments, and resolve complex issues efficiently. Oversaw cloud infrastructure across Microsoft 365, AWS, and Azure, managing identity access, virtual machine deployments, storage, and billing. Maintained local networks, databases, print servers, and mobile device management policies to ensure continuous uptime and security. Directed backup and disaster recovery strategies using Veeam while handling hardware procurement, vendor quoting, and machine configurations for company staff.",
        bullets: &[
            "Provided support across Microsoft, Linux, and Apple hardware and software environments, including Windows Server 2008R2-2019 and Linux Ubuntu.",
            "Developed and executed PowerShell and Bash scripts for systems deployment and troubleshooting.",
            "Supported Microsoft 365, Active Directory, Azure AD, Intune, Exchange, and SharePoint ecosystems.",
            "Configured and managed printer deployments across local and TCP/IP network environments.",
            "Utilized Veeam and Windows Backup solutions to perform routine system restorations and disaster recovery operations.",
            "Performed database administration across SQL, MySQL, phpMyAdmin, and MongoDB platforms.",
            "Maintained enterprise LAN/WAN infrastructure and managed corporate Wi-Fi networks.",
            "Provided technical support and mobile device management (MDM) configuration for Android and iOS devices.",
            "Deployed and managed virtual machines and infrastructure within the Azure Portal.",
            "Managed cloud infrastructure across AWS environments, including EC2, S3 buckets, IAM roles, policies, and billing metrics.",
            "Evaluated and recommended hardware and software configurations for operator workstations.",
            "Managed vendor quoting and procurement for corporate desktops, laptops, servers, and hardware accessories."
        ]
    },
    JobData {
        id: 4,
        img_src: "/images/logistics.png",
        alt: "Logistics and Automation Workflows",
        title: "Logistics Specialist & Tier I Support",
        company: "A&S Logistics",
        period: "Oct 2012 – Feb 2018",
        description: "Provided Tier I technical support to resolve internal hardware, software, and networking issues while escalating complex tickets to senior engineers when necessary. Supported supply chain operations by helping team members navigate logistics software, fix data errors, and resolve account access issues. Maintained the internal IT knowledge base with step-by-step troubleshooting guides to speed up resolution times and empower user self-service. Partnered with operations teams during system outages to keep supply chain disruptions to a minimum. Managed technical onboarding for new employees, handling account creation, laptop configurations, and tool training.",
        bullets: &[
            "Delivered Tier I IT support for internal staff, resolving hardware, software, and network issues, and escalating technical problems to higher support tiers when needed.",
            "Supported logistics software, assisting team members with login/access issues, data entry errors, and system navigation.",
            "Maintained and updated the internal knowledge base for common IT issues, contributing to faster resolution times and improved team self-sufficiency.",
            "Collaborated with IT and operations teams to troubleshoot system outages and ensure minimal disruption to supply chain and inventory processes.",
            "Provided technical onboarding for new hires, including account setup, device configuration, and training on company tools and software."
        ]
    }
];

// --- MAIN SHOWCASE LANDING ---
#[component]
fn Home() -> Element {
    let mut selected_card = use_signal(|| None::<usize>);

    rsx! {
        main { class: "max-w-6xl mx-auto px-6 pt-0 pb-16 space-y-32",
            
            // Hero Block
            section { class: "max-w-6xl space-y-8",
                h1 { class: "text-5xl md:text-8xl font-black tracking-tight text-neutral-900 max-w-4xl flex flex-col items-start gap-y-1 md:gap-y-1.5",
                    span { class: "bg-neutral-100 px-2 pt-1 pb-2 md:pb-3 rounded-lg inline-block leading-none",
                        "System"
                    }
                    span { class: "bg-neutral-100 px-4 pt-2 pb-3 rounded-xl inline-block leading-none",
                        "Administrator"
                    }
                    span { class: "text-transparent bg-clip-text bg-gradient-to-r from-emerald-600 via-teal-600 to-cyan-600 inline-block pt-2 md:pt-3", 
                        "Software Engineer." 
                    }
                }
                div { class: "text-xs font-bold uppercase tracking-widest text-neutral-400 flex flex-wrap gap-4 items-center",
                    span { "IT Manager/Specialist" }
                    span { class: "text-neutral-300", "•" }
                    span { "System Administrator" }
                    span { class: "text-neutral-300", "•" }
                    span { "Software Engineer/Video Game Developer" }
                    span { class: "text-neutral-300", "•" }
                    span { class: "text-neutral-600", "Denver, CO" }
                }
                
                p { class: "text-lg md:text-xl text-neutral-600 max-w-6xl font-light leading-relaxed",
                    "Versatile and security-focused system administrator and software engineer delivering end-to-end infrastructure support and automation, from on-premise systems to cloud platforms including building automation systems such as Delta EnteliWeb Server and Tridium N4. Strong expertise in Windows Server (2008–2022), Linux server, Hyper-V, entire Office 365 tenant, Fortinet firewall, and Cisco Meraki networks. Software development tools include Rust, Python, Node js, React js including CI/CD piping. Proven success in cyberattack recovery, Hyper-V VM recovery, VPN with SAML/SSO setups, Synology configuration, and SaaS services administration. Demonstrated proficiency in Azure VM setup and Azure AD administration, AWS administration, configuration, and management. Committed to secure and scalable IT environments and rapid response to operational needs with NIST 800-171 compliance requirements."
                }
            }

            // Core Technical Masteries (Grid Badges)
            section { id: "skills", class: "space-y-6",
                h2 { class: "text-xs uppercase tracking-widest text-neutral-400 font-bold border-b border-neutral-100 pb-4", "Knowledge tags" }
                div { class: "flex flex-wrap gap-2 text-xs font-mono font-medium text-neutral-700",
                    for skill in [
                        "Windows Server (2008-2022)", "Linux Ubuntu LTS", "ProxmoxVE", "Hyper-V Virtualization", 
                        "Fortinet FortiGate", "Cisco Meraki LAN/WAN", "Intune Deployment", "Azure & Entra ID", "AWS EC2/S3/IAM", "NIST 800-171 Compliance", "SAML/SSO VPN", "Veeam & Synology Core", "PowerShell & Bash Scripting", "Ansible & Python Automation", "Node.js & React.js", "Rust & WebAssembly", "SQL/MySQL/MongoDB & PostgesSQL", "Delta BAS EnteliWeb administration", "Tridium N4 administration"
                    ] {
                        span { class: "bg-neutral-50 border border-neutral-200/60 px-3 py-1.5 rounded-full shadow-sm", "{skill}" }
                    }
                }
            }

            // --- INTERACTIVE PHOTO MATRIX LAYER ---
            section { id: "experience", class: "space-y-6 transition-all duration-300",
                h2 { class: "text-xs uppercase tracking-widest text-neutral-400 font-bold border-b border-neutral-100 pb-4 mb-4", "Professional Experience" }
                
                if selected_card().is_some() {
                    div { 
                        role: "button",
                        class: "text-xs font-mono font-black tracking-widest text-neutral-900 hover:text-emerald-600 bg-neutral-100 hover:bg-neutral-200/70 border-2 border-neutral-300 px-5 py-2.5 rounded-full transition-all duration-200 uppercase flex items-center gap-2 mb-6 cursor-pointer outline-none select-none w-fit shadow-md",
                        onclick: move |_| selected_card.set(None),
                        "X Close"
                    }
                }

                div { 
                    class: if selected_card().is_some() { "grid grid-cols-1 gap-6" } else { "grid grid-cols-1 md:grid-cols-2 gap-6" },
                    
                    for job in JOBS.iter() {
                        if selected_card().is_none() || selected_card() == Some(job.id) {
                            div { 
                                key: "{job.id}",
                                class: "overflow-hidden rounded-2xl border border-neutral-100 bg-neutral-50 shadow-sm job-card-transition",
                                
                                if selected_card() == Some(job.id) {
                                    div { class: "p-8 md:p-12 bg-white space-y-6 fade-in-content",
                                        
                                        div { class: "flex flex-col md:flex-row justify-between items-start md:items-center pb-4 gap-4",
                                            div {
                                                h3 { class: "text-3xl font-black text-neutral-900 tracking-tight", "{job.title}" }
                                                p { class: "text-base text-neutral-500 font-medium mt-1", "{job.company}" }
                                            }
                                            span { class: "text-xs font-mono font-bold whitespace-nowrap bg-transparent border-0 border-none p-0 shadow-none", 
                                                "{job.period}" 
                                            }
                                        }
                                        
                                        div { class: "space-y-6 stagger-item",
                                            p { class: "text-neutral-600 font-light leading-relaxed text-base max-w-4xl", "{job.description}" }
                                            ul { class: "list-disc list-inside text-sm text-neutral-500 space-y-2.5 pl-2 font-light max-w-4xl",
                                                for bullet in job.bullets.iter() {
                                                    li { "{bullet}" }
                                                }
                                            }
                                        }
                                        
                                    }
                                } else {
                                    div { 
                                        role: "button",
                                        class: "w-full aspect-video bg-neutral-100 overflow-hidden cursor-pointer group rounded-2xl select-none relative block",
                                        onclick: move |_| selected_card.set(Some(job.id)),
                                        img { 
                                            class: "w-full h-full object-cover transition-transform duration-500 group-hover:scale-[1.01]",
                                            src: "{job.img_src}",
                                            alt: "{job.alt}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // --- CREDENTIALS & FOUNDATION ---
            section { id: "education", class: "grid grid-cols-1 md:grid-cols-2 gap-12 pt-8",
                div { class: "space-y-4",
                    h2 { class: "text-xs uppercase tracking-widest text-neutral-400 font-bold border-b border-neutral-100 pb-4", "Academic Foundations" }
                    div {
                        h4 { class: "text-lg font-bold text-neutral-900", "Bachelor of Arts" }
                        p { class: "text-sm text-neutral-500 font-medium", "Metropolitan State University" }
                    }
                }
                div { class: "space-y-4",
                    h2 { class: "text-xs uppercase tracking-widest text-neutral-400 font-bold border-b border-neutral-100 pb-4", "Specialized Endorsements" }
                    div {
                        h4 { class: "text-lg font-bold text-neutral-900", "Ruby on Rails Developer Certification" }
                        p { class: "text-sm text-neutral-500 font-medium", "DaVinci Coders — Louisville, CO" }
                    }
                }
            }
        }
    }
}

// --- PROJECT DEEP DIVE LAYER ---
#[component]
fn ProjectDetail(name: String) -> Element {
    rsx! {
        main { class: "max-w-4xl mx-auto px-6 py-24 space-y-12",
            Link { to: Route::Home {}, class: "text-xs font-bold tracking-widest uppercase text-neutral-400 hover:text-neutral-900 transition-colors", "← Back to Work" }
            h1 { class: "text-4xl md:text-6xl font-black tracking-tight text-neutral-900 capitalize", "{name}" }
            div { class: "w-full h-[1px] bg-neutral-200" }
            p { class: "text-lg text-neutral-600 font-light leading-relaxed",
                "Deep dive logs for the {name} module ecosystem. This canvas isolates documentation specs mapping directly back to systems execution pipelines."
            }
        }
    }
}