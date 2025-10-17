import { CustomElement } from "./shared.js";

class NavBar extends CustomElement {
  constructor() {
    super()
  }

  async connectedCallback() {
    const shadowRoot = await this.init("nav-bar.html", "nav-bar.css")
    shadowRoot.querySelectorAll(".nav-bar-elem")
      .forEach(link => this.markIfActive(link))
  }

  markIfActive(link) {
    // Normalize the href attribute if one was provided without the slash
    let href = link.getAttribute("href")
    if (!href.startsWith("/")) {
      href = "/" + href
    }

    const isCurrentPathActive = href === window.location.pathname
    const isRoot = window.location.pathname === "/"

    if (isCurrentPathActive || (isRoot && href === "/index.html")) {
      link.classList.add("active")
    }
  }
}

customElements.define("nav-bar-template", NavBar)
