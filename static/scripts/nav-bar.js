class NavBar extends HTMLElement {
  template;

  constructor() {
    super()

    this.template = document.createElement("template")
  }

  async connectedCallback() {
    const shadowRoot = this.attachShadow({ mode: "closed" })

    const htmlResp = await fetch("templates/nav-bar.html")
    this.template.innerHTML = await htmlResp.text()

    shadowRoot.appendChild(this.template.content)
    shadowRoot.querySelectorAll(".nav-bar-elem")
      .forEach(link => this.markIfActive(link))
  }

  markIfActive(link) {
    // Normalize the href attribute if one was providen without the slash
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

customElements.define("nav-bar", NavBar)
