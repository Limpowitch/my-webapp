
function homeButton(){
    console.log('Button was clicked!');
    window.location.href = '/hello'
}

function categoryButton(category: string) {
    fetch('/set_category', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({category}),
    })
    .then(response => {
        if (response.ok) {
            // Get the HTML content returned from the Rust backend
            return response.text(); 
        } else {
            console.error('Failed to set category');
            throw new Error('Failed to set category');
        }
    })
    .then(htmlContent => {
        // Replace the current page content with the new HTML content
        document.body.innerHTML = htmlContent;
    })
    .catch(error => {
        console.error('Error: ', error);
    });
}


