

document.addEventListener('DOMContentLoaded', function() {
    const toggleButton = document.querySelector('.sidebar-toggle');
    const sidebar = document.querySelector('.sidebar');
    const closeButton = document.querySelector('.close-sidebar');

    toggleButton.addEventListener('click', function() {
        sidebar.classList.toggle('open');
    });

    closeButton.addEventListener('click', function() {
        sidebar.classList.remove('open');
    });
});


