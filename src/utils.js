(function(){
function submit(searchForm) {
    var form = document.getElementById('search-form');
    var text = form.elements["author"];
    console.log("hi from submit()");
    console.log(text);
};
})();