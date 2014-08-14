
$(function() {
  window.WebSocket=window.WebSocket || window.MozWebSocket || false;
  if(!window.WebSocket){
    alert("No WebSocket Support");
  }else {
    var ws=new WebSocket("ws://"+location.host+"/now");
    var now_el=document.getElementById("now");
    ws.onmessage=function(e){
        now_el.innerHTML=e.data;
    }
    ws.onclose=function(){
        now_el.innerHTML='closed';
    }
  }
});
