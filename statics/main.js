import Todos from './render.js';

let newTodoInput = document.getElementById('new-todo-name');

(async () => {
    let base = document.createElement('div');
    document.body.append(base);
    let todos = new Todos(base, document.querySelector('#todo-template'));
    await todos.fetch_all();
    todos.render();

    newTodoInput.addEventListener('keypress', async e => {
        if (e.key === 'Enter') {
            //@ts-ignore
            if (e.target.value !== '') {
                //@ts-ignore
                let name = e.target.value;
                //@ts-ignore
                e.target.value = '';
                let res = await fetch('/api/todo', {
                    method: 'POST',
                    body: JSON.stringify({ name: name, is_checked: false }),
                });
                let json = await res.json();
                todos.add_todo_callback(json);
            }
        }
    });
})();
