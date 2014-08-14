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

$(function() {
  React.renderComponent(
    <MinionDetails url={'/api/minions/' + $('#minion-details').data('minion-id')} />,
    document.getElementById('minion-details')
  );
});
