$(() => {
    var conn = null;
    
    const update_status = (status) => {
	$('#status').html(status);
	$('#time').html(new Date().toLocaleString())
    }

    const connect = () => {
	disconnect();
	const wsUri = (window.location.protocol=='https:'&&'wss://'||'ws://')+window.location.host + '/ws/';
	conn = new WebSocket(wsUri);

	conn.onmessage = function(e) {
	    console.log(e)
	    update_status("someting") 
	};
    }

    const disconnect = () => {
        if (conn != null) {
          conn.close();
          conn = null;
        }
      }

    connect()
});
