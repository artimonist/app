import { HelloPlanet } from './hello-planet.mjs';

export class HelloWorld extends HTMLElement {
  constructor() {
    super()
  }
  connectedCallback() {
    // Attach a shadow root to the element.
    let shadowRoot = this.attachShadow({ mode: 'open' });
    ['Mercury', 'Venus', 'Earth', 'Mars', 'Jupiter', 'Saturn', 'Uranus', 'Neptune'].forEach(planet => {
      let el = document.createElement('hello-planet')
      el.setAttribute('planet', planet);
      shadowRoot.appendChild(el)
    })
  }
}

customElements.define('hello-world', HelloWorld);
customElements.define('hello-planet', HelloPlanet);