class NavBar extends HTMLElement {
  constructor() {
    super()
  }

  async connectedCallback() {
    const htmlResp = await fetch("templates/nav-bar.html")

    let template = document.createElement("template")
    template.innerHTML = await htmlResp.text()

    const shadowRoot = this.attachShadow({ mode: "closed" })
    shadowRoot.appendChild(template.content)
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
