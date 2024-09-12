function homeButton() {
    console.log('Button was clicked!');
    window.location.href = '/hello';
}
function categoryButton(category) {
    fetch('/set_category', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ category: category }),
    })
        .then(function (response) {
        if (response.ok) {
            window.location.href = '/categories';
        }
        else {
            console.error('Failed to set category');
        }
    })
        .catch(function (error) {
        console.error('Error: ', error);
    });
}
