import Renderer from './render.js';

(async () => {
    let res = await fetch('/api/todos');
    let json = await res.json();
    console.log(json);

    let base = document.createElement('div');
    document.body.append(base);
    let r = new Renderer(base);
    r.render(json);
})();
