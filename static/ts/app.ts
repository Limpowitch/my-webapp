
function homeButton(){
    console.log('Button was clicked!');
    window.location.href = '/hello'
}

function categoryButton(category: string){
    fetch('/set_category', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({category}),
    })
    .then(response => {
        if (response.ok) {
            window.location.href = '/categories';
        } else {
            console.error('Failed to set category');
        }
    })
    .catch(error => {
        console.error('Error: ', error);
    })
}
