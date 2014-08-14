/** @jsx React.DOM */

var MinionsTable = React.createClass({
  loadMinionsFromServer: function() {
    console.log("loading!!::::!::...");
    $.ajax({
      url: this.props.url,
      dataType: 'json',
      success: function(data) {
        this.setState({data: data});
      }.bind(this),
      error: function(xhr, status, err) {
        console.error(this.props.url, status, err.toString());
      }.bind(this)
    });
  },
  getInitialState: function() {
    return {data: []};
  },
  componentDidMount: function() {
    this.loadMinionsFromServer();
  },
  render: function() {
    var rows = this.state.data.map(function (minion) {
      return (
        <tr>
          <td><a href={'/minions/' + minion.id}>{minion.id}</a></td>
          <td>{minion.osfullname}</td>
          <td>{minion.kernelrelease}</td>
        </tr>
      );
    });
    return (
      <table className='table table-striped'>
        <thead>
          <tr>
            <th>#</th>
            <th>OS</th>
            <th>Kernel</th>
          </tr>
         </thead>
         <tbody>
            {rows}
        </tbody>
      </table>
    );
  }
});

React.renderComponent(
  <MinionsTable url='/api/minions' />,
  document.getElementById('minions-table')
);
