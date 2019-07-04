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

export default class Todos {
    /**
     * @param {HTMLElement} entryPoint
     * @param {HTMLTemplateElement} template
     */
    constructor(entryPoint, template) {
        this.base = entryPoint;
        this.template = template.content;
        /** @type {Todo[]} */
        this.todos = [];
    }

    /**
     * @param {number} id
     * @param {boolean} checked
     */
    async change_checked(id, checked) {
        await fetch(`/api/todo/${id}`, {
            method: 'PATCH',
            body: JSON.stringify({ is_checked: checked }),
        });
        await this.update_todos();
    }

    /**
     * @param {number} id
     */
    async delete(id) {
        await fetch(`/api/todo/${id}`, {
            method: 'DELETE',
        });
        await this.update_todos();
    }

    async update_todos() {
        await this.fetch_all();
        this.render();
    }

    async fetch_all() {
        let res = await fetch('/api/todos');
        let json = await res.json();
        this.todos = json;
    }

    render() {
        this.base.childNodes.forEach(v => v.remove());
        let base = document.createElement('ul');
        this.todos.forEach(v => {
            let todo = this.template.cloneNode(true);
            //@ts-ignore
            let name = todo.querySelector('.todo-name');
            if (v.is_checked) {
                name.classList.add('todo-done');
            }
            name.innerText = v.name;
            //@ts-ignore
            todo.querySelector('.todo-delete').addEventListener('click', () => {
                this.delete(v.id);
            });
            //@ts-ignore
            let checked = todo.querySelector('.todo-checked');
            checked.checked = v.is_checked;
            checked.addEventListener('click', e => {
                this.change_checked(v.id, e.target.checked);
            });
            base.append(todo);
        });
        this.base.append(base);
    }
}
