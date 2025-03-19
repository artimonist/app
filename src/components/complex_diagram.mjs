let css = `
:host {
  display: inline-grid;
  grid-template-columns: repeat(7, 1fr);
  grid-template-rows: repeat(7, 1fr);
  place-content: center;
  gap: 3px;
  border: 1px solid #e5e4e9;
}
.cell {
  width: 120px;
  height: 60px;
  resize: none;
  text-align: center;

  overflow: hidden;
  border: 1px solid #e5e4e9;
  border-radius: 5px;
  padding-top: 10px;

  font-size: 36px;
  line-height: 1.2;
  text-wrap-style: pretty;
}
.cell:hover {
  box-shadow: 5px 5px 10px #aaaaaa, -5px -5px 10px #ffffff;
}
.cell:nth-of-type(7n+4),
.cell:nth-of-type(n + 22):nth-of-type(-n + 28),
.cell:nth-of-type(28n + 9),
.cell:nth-of-type(28n + 13) {
  background-color: #f5f5f5;
}
.cell:valid {
  box-shadow: inset -1px -1px 3px #ffffff, inset 1px 1px 3px #aaaaaa;
}`;
let cell = `<textarea required maxlength="50" placeholder=" " class="cell" spellcheck="false"></textarea>`;

const unicode_limit = (s, n) => Array.from(s ?? '').length > 1 ? Array.from(s).slice(0, n).join('') : s;
const unicode_tip = (s) => Array.from(s ?? '').map(c => `\\u\{${c.codePointAt(0).toString(16)}\}`).join('');
const adjust_font = (t, n) => { do { t.style.fontSize = `${n--}px`; } while (t.clientHeight < t.scrollHeight) };

class ComplexDiagram extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  connectedCallback() {
    let shadow = this.shadowRoot;
    shadow.innerHTML = `<style>${css}</style>${cell.repeat(49)}`;
    this.$cells = Array.from(shadow.querySelectorAll('.cell'));

    shadow.addEventListener('input', (e) => {
      // prevent newline character, newlines are not allowed.
      let s = e.target.value.replace(/\r?\n/gi, '');
      // limit characters length to 20. (most words are less than 20 characters in length)
      e.target.value = unicode_limit(s, 20);
      e.target.title = unicode_tip(e.target.value);
      // auto ajust font-size, none characters hide.
      adjust_font(e.target, 36);
    });

    // dispatch custom event
    shadow.addEventListener('change', (e) => {
      this.dispatchEvent(
        new CustomEvent('onchange', { detail: e })
      );
    });
  }

  is_empty() {
    return this.$cells.every(e => e.value === '');
  }

  get values() {
    return JSON.stringify(this.$cells.map(e => e.value));
  }

  set values(value) {
    JSON.parse(value).forEach((v, i) => {
      let s = unicode_limit(v, 20);
      this.$cells[i].value = s;
      this.$cells[i].title = unicode_tip(s);
    });
  }

  static get observedAttributes() {
    return ['values'];
  }

  attributeChangedCallback(name, oldVal, newVal) {
    this[name] = newVal;
  }
}
customElements.define('complex-diagram', ComplexDiagram)
