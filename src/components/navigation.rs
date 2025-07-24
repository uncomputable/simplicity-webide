use leptos::{component, create_signal, view, IntoView, SignalGet, SignalSet};

#[component]
pub fn Navigation() -> impl IntoView {
    let (mobile_menu_open, set_mobile_menu_open) = create_signal(false);
    let (docs_open, set_docs_open) = create_signal(false);
    let (packages_open, set_packages_open) = create_signal(false);
    let (community_open, set_community_open) = create_signal(false);

    let toggle_mobile_menu = move |_| {
        set_mobile_menu_open.set(!mobile_menu_open.get());
        if !mobile_menu_open.get() {
            set_docs_open.set(false);
            set_packages_open.set(false);
            set_community_open.set(false);
        }
    };

    let toggle_docs = move |_| {
        set_docs_open.set(!docs_open.get());
    };

    let toggle_packages = move |_| {
        set_packages_open.set(!packages_open.get());
    };

    let toggle_community = move |_| {
        set_community_open.set(!community_open.get());
    };

    let close_mobile_menu = move |_| {
        set_mobile_menu_open.set(false);
        set_docs_open.set(false);
        set_packages_open.set(false);
        set_community_open.set(false);
    };

    view! {
        <nav class="navigation">
            <div class="navigation-container">
                <a href="/" class="navigation-logo-link">
                    <img src="images/simplicity-logo.svg" alt="Simplicity Logo" class="navigation-logo" />
                </a>

                // Desktop Navigation
                <div class="navigation-desktop">
                    <div class="navigation-dropdown">
                        <span class="navigation-dropdown-label">"Docs"</span>
                        <div class="navigation-dropdown-content">
                            <a href="https://docs.simplicity-lang.org/" target="_blank" rel="noopener noreferrer" class="navigation-dropdown-item">
                                <div class="navigation-dropdown-title">"SimplicityHL"</div>
                                <div class="navigation-dropdown-description">"Small description here about this item"</div>
                            </a>
                            <a href="https://github.com/BlockstreamResearch/SimplicityHL/tree/master/examples" target="_blank" rel="noopener noreferrer" class="navigation-dropdown-item">
                                <div class="navigation-dropdown-title">"Contract Examples"</div>
                                <div class="navigation-dropdown-description">"Small description here about this item"</div>
                            </a>
                            <a href="https://docs.rs/simfony-as-rust" target="_blank" rel="noopener noreferrer" class="navigation-dropdown-item">
                                <div class="navigation-dropdown-title">"Jet Documentation"</div>
                                <div class="navigation-dropdown-description">"Small description here about this item"</div>
                            </a>
                        </div>
                    </div>

                    <div class="navigation-dropdown">
                        <span class="navigation-dropdown-label">"Packages"</span>
                        <div class="navigation-dropdown-content">
                            <a href=" https://ide.simplicity-lang.org/" class="navigation-dropdown-item">
                                <div class="navigation-dropdown-title">"SimplicityHL IDE"</div>
                                <div class="navigation-dropdown-description">"Small description here about this item"</div>
                            </a>
                            <a href="https://github.com/BlockstreamResearch/simplicity" target="_blank" rel="noopener noreferrer" class="navigation-dropdown-item">
                                <div class="navigation-dropdown-title">"Simplicity"</div>
                                <div class="navigation-dropdown-description">"Small description here about this item"</div>
                            </a>
                            <a href="https://github.com/BlockstreamResearch/SimplicityHL" target="_blank" rel="noopener noreferrer" class="navigation-dropdown-item">
                                <div class="navigation-dropdown-title">"SimplicityHL Compiler"</div>
                                <div class="navigation-dropdown-description">"Small description here about this item"</div>
                            </a>
                        </div>
                    </div>

                    <a href="https://simplicity-lang.org/content" class="navigation-link">"Learn"</a>

                    <div class="navigation-dropdown navigation-dropdown-community">
                        <span class="navigation-dropdown-label">"Community"</span>
                        <div class="navigation-dropdown-content navigation-dropdown-content-community">
                            <a href="https://x.com/blksresearch" target="_blank" rel="noopener noreferrer" class="navigation-dropdown-item navigation-dropdown-item-community">
                                <img src="images/x.svg" alt="Twitter" class="navigation-dropdown-icon" />
                                <span class="navigation-dropdown-title">"Twitter"</span>
                            </a>
                            <a href="https://t.me/simplicity_community" target="_blank" rel="noopener noreferrer" class="navigation-dropdown-item navigation-dropdown-item-community">
                                <img src="images/telegram.svg" alt="Telegram" class="navigation-dropdown-icon" />
                                <span class="navigation-dropdown-title">"Telegram"</span>
                            </a>
                        </div>
                    </div>
                </div>

                // Mobile Menu Button
                <button class="navigation-mobile-button" on:click=toggle_mobile_menu>
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/>
                    </svg>
                </button>
            </div>

            // Mobile Menu
            <div 
                class="navigation-mobile-menu"
                class:active=move || mobile_menu_open.get()
            >
                <div class="navigation-mobile-section">
                    <button class="navigation-mobile-dropdown-button" on:click=toggle_docs>"Docs" <i class="fa fa-chevron-down"></i></button>
                    <div 
                        class="navigation-mobile-dropdown" 
                        class:active=move || docs_open.get()
                    >
                        <a href="https://docs.simplicity-lang.org/" target="_blank" rel="noopener noreferrer" class="navigation-mobile-dropdown-item" on:click=close_mobile_menu>"SimplicityHL"</a>
                        <a href="https://github.com/BlockstreamResearch/SimplicityHL/tree/master/examples" target="_blank" rel="noopener noreferrer" class="navigation-mobile-dropdown-item" on:click=close_mobile_menu>"Contract Examples"</a>
                        <a href="https://docs.rs/simfony-as-rust" target="_blank" rel="noopener noreferrer" class="navigation-mobile-dropdown-item" on:click=close_mobile_menu>"Jet Documentation"</a>
                    </div>
                </div>

                <div class="navigation-mobile-section">
                    <button class="navigation-mobile-dropdown-button" on:click=toggle_packages>"Packages" <i class="fa fa-chevron-down"></i></button>
                    <div 
                        class="navigation-mobile-dropdown" 
                        class:active=move || packages_open.get()
                    >
                        <a href=" https://ide.simplicity-lang.org/" class="navigation-mobile-dropdown-item" on:click=close_mobile_menu>"SimplicityHL IDE"</a>
                        <a href="https://github.com/BlockstreamResearch/simplicity" target="_blank" rel="noopener noreferrer" class="navigation-mobile-dropdown-item" on:click=close_mobile_menu>"Simplicity"</a>
                        <a href="https://github.com/BlockstreamResearch/SimplicityHL" target="_blank" rel="noopener noreferrer" class="navigation-mobile-dropdown-item" on:click=close_mobile_menu>"SimplicityHL Compiler"</a>
                    </div>
                </div>

                <a href="https://simplicity-lang.org/content" class="navigation-mobile-link" on:click=close_mobile_menu>"Content"</a>

                <div class="navigation-mobile-section">
                    <button class="navigation-mobile-dropdown-button" on:click=toggle_community>"Community" <i class="fa fa-chevron-down"></i></button>
                    <div 
                        class="navigation-mobile-dropdown" 
                        class:active=move || community_open.get()
                    >
                        <a href="https://x.com/blksresearch" target="_blank" rel="noopener noreferrer" class="navigation-mobile-dropdown-item navigation-mobile-dropdown-item-community" on:click=close_mobile_menu>
                            <img src="images/icons/x.svg" alt="Twitter" class="navigation-dropdown-icon" />
                            <span>"Twitter"</span>
                        </a>
                        <a href="https://t.me/simplicity_community" target="_blank" rel="noopener noreferrer" class="navigation-mobile-dropdown-item navigation-mobile-dropdown-item-community" on:click=close_mobile_menu>
                            <img src="images/icons/telegram.svg" alt="Telegram" class="navigation-dropdown-icon" />
                            <span>"Telegram"</span>
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}