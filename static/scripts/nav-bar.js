import { CustomElement } from "./shared.js";

class NavBar extends CustomElement {
  constructor() {
    super()
  }

  async connectedCallback() {
    const shadowRoot = await this.init("nav-bar.html", "nav-bar.css")
    shadowRoot.querySelectorAll("#nav-bar li>a")
      .forEach(link => this.markIfActive(link))
  }

  markIfActive(link) {
    let href = link.getAttribute("href")

    const isCurrentPathActive = window.location.pathname.includes(href)
    const isRoot = window.location.pathname === "/"

    if (isCurrentPathActive || (isRoot && href === "/index.html")) {
      link.classList.add("active")
    }
  }
}

customElements.define("nav-bar-template", NavBar)
