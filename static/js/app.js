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
            // If successful, the server will automatically redirect to /category
            window.location.href = '/category';
        }
        else {
            console.error('Failed to set category');
            throw new Error('Failed to set category');
        }
    })
        .catch(function (error) {
        console.error('Error: ', error);
    });
}
