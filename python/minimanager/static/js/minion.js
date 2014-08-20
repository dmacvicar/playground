/** @jsx React.DOM */

var MinionDetails = React.createClass({
  loadMinionFromServer: function() {
    console.log("loading.....");
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
    this.loadMinionFromServer();
  },
  render: function() {
    return (
      <div className='panel panel-default'>
        <div className='panel-heading'>{this.state.data.id}</div>
        <div className='panel-body'>
          <p>...</p>
        </div>

        <ul className='list-group'>
          <li className='list-group-item'>{this.state.data.osfullname} {this.state.data.osrelease}</li>
          <li className='list-group-item'>{this.state.data.kernel} {this.state.data.kernelrelease}</li>
        </ul>
      </div>
    );
  }
});

var MinionPackages = React.createClass({
  loadPackagesFromServer: function() {
    console.log("loading packages.....");
    $.ajax({
      type: 'POST',
      url: '/api/lowstate',
      data: {
        fun: 'pkg.list_pkgs',
        tgt: this.props.minion,
        mode: 'sync'
      },
      dataType: 'json',
      success: function(data) {
        this.setState({data: data[this.props.minion]});
      }.bind(this),
      error: function(xhr, status, err) {
        console.error(this.props.url, status, err.toString());
      }.bind(this),
    });
  },
  getInitialState: function() {
    return {data: {}};
  },
  componentDidMount: function() {
    this.loadPackagesFromServer();
  },
  render: function() {
    var rows = [];
    for (var pkg in this.state.data) {
      rows.push(
        <tr>
          <td>{pkg}</td>
          <td>{this.state.data[pkg]}</td>
        </tr>
      );
    }
    return (
      <table className='table table-striped'>
        <thead>
          <tr>
            <th>#</th>
            <th>Version</th>
          </tr>
         </thead>
         <tbody>
            {rows}
        </tbody>
      </table>
    );
  }
});


$(function() {

  function showDetailsTab() {
    React.renderComponent(
      <MinionDetails url={'/api/minions/' + $('#minion-content').data('minion-id')} />,
      document.getElementById('minion-content')
    );
  }

  function showSoftwareTab() {
    React.renderComponent(
      <MinionPackages minion={$('#minion-content').data('minion-id')} />,
      document.getElementById('minion-content')
    );
  }

  $('#minion-details-tab').click(showDetailsTab);
  $('#minion-software-tab').click(showSoftwareTab);
  showDetailsTab();
});


