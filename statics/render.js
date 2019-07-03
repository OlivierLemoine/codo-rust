export class Todo {
    /**
     * @param {number} id
     * @param {string} name
     * @param {boolean} is_checked
     */
    constructor(id, name, is_checked) {
        this.id = id;
        this.name = name;
        this.is_checked = is_checked;
    }
}

export default class {
    /**
     * @param {HTMLElement} entryPoint
     */
    constructor(entryPoint) {
        this.base = entryPoint;
    }

    /**
     * @param {Todo[]} todos
     */
    render(todos) {
        this.base.childNodes.forEach(v => v.remove());
        todos.forEach(v => {
            this.base.append(v.name);
        });
    }
}
