// ordering of imports does not matter
import { HelloPlanet } from './hello-planet.mjs';
import { HelloWorld } from './hello-world.mjs';

customElements.define('hello-world', HelloWorld);
customElements.define('hello-planet', HelloPlanet);

// this will typically be handled by a client side router (e.g. Navigo)
// let el = document.createElement('hello-world');
// let body = document.querySelector('body');
// body.appendChild(el);