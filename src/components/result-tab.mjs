let css = `
.tabs {
  display: flex;
  cursor: pointer;
}
.tab {
  padding: 10px;
  border: 1px solid #ccc;
  border-bottom: none;
  background: #f1f1f1;
}
.tab-content {
  border: 1px solid #ccc;
  padding: 10px;
}
.active {
  background: #fff;
  border-bottom: 1px solid #fff;
}`;
let html = `
<div class="tabs">
  <div class="tab active" data-tab="1">Tab 1</div>
  <div class="tab" data-tab="2">Tab 2</div>
  <div class="tab" data-tab="3">Tab 3</div>
</div>
<div class="tab-content" id="tab-content-1">Content for Tab 1</div>
<div class="tab-content" id="tab-content-2" style="display: none;">Content for Tab 2</div>
<div class="tab-content" id="tab-content-3" style="display: none;">Content for Tab 3</div>`;

class ResultTab extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  connectedCallback() {
    this.render();
  }

  render() {
    this.shadowRoot.innerHTML = `<style>${css}</style>${html}`;

    this.shadowRoot.querySelectorAll('.tab').forEach(tab => {
      tab.addEventListener('click', (event) => this.onTabClick(event));
    });
  }

  onTabClick(event) {
    const selectedTab = event.target.getAttribute('data-tab');
    this.shadowRoot.querySelectorAll('.tab').forEach(tab => {
      tab.classList.toggle('active', tab.getAttribute('data-tab') === selectedTab);
    });
    this.shadowRoot.querySelectorAll('.tab-content').forEach(content => {
      content.style.display = content.id === `tab-content-${selectedTab}` ? 'block' : 'none';
    });
  }
}

customElements.define('result-tab', ResultTab);