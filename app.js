'use strict';

const e = React.createElement;
let achievements = [];
let initialized = false;
class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {valueTime: 0};
    this.state = {valueName: ''};
    this.state = {valueAttempts: 0};
    this.state = {valueFactBody: ''};
    this.state = {valueFactTitle: ''};
    this.state = {valueFactLink: ''};
    this.state = {valueAchievements: []};
    this.state = {authorized: true};
    //this.state = {lastElemet: ''};
    
    this.handleClickNewAttempt = this.handleClickNewAttempt.bind(this);
    this.handleClickNextFact = this.handleClickNextFact.bind(this);
  }
  
  handleClickNewAttempt() {
    fetch('/newattempt', {
      method: 'GET',
    })
    .then(this.setState({valueAttempts: this.state.valueAttempts+1, valueTime: 0}));
  }

  handleClickNextFact() {
    fetch('/fact', {
      method: 'GET',
    })
    .then((response) => {
      return response.json();
    })
    .then((data) => {
      console.log(data);
      this.setState({
        valueFactBody: data.body,
        valueFactTitle: data.title,
        valueFactLink: data.link
      });
    });
  }
  componentDidMount() {
    setInterval(() => {
      this.setState({valueTime: this.state.valueTime+1});
    }, 1000);
    /* setTimeout(() => {
      var hiddenElement = document.getElementById("6");
      console.log(hiddenElement);
      hiddenElement.scrollIntoView({block: "center", behavior: "smooth"});
    }, 1000); */
    fetch('/initialize', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      }, 
      
    }).then((response) => {
      return response.json();
    })
    .then((data) => {
      this.setState({
        valueTime: data.time.secs,
        valueName: data.name,
        valueAttempts: data.attempts,
        valueFactBody: data.fact.body,
        valueFactTitle: data.fact.title,
        valueFactLink: data.fact.link,
        valueAchievements: data.achievements
      });
      console.log(data.achievements);
      initialized = true;
    });
  }
  render() {
    if (initialized) {
      achievements = this.state.valueAchievements.map((a) => e ("div", {key: a.id, id: a.id, className: "col-4", style: {minWidth: 300 + 'px', display: 'inline-block'}}, 
        e ("p", {className: "align-top", style: {whiteSpace: 'normal', height: 100 + 'px'}}, a.body),
        e ("div", {className: "progress", style: {height: 5 + 'px'}}, e ("div", {className: "progress-bar bg-warning", role: "progressbar", style: {width: this.state.valueTime/a.duration*100 + '%'}, 'aria-valuenow': "25", 'aria-valuemin': "0", 'aria-valuemax': "100"})),
        e ("p", {className: "text-left"}, completed(a.duration,this.state.valueTime)))
      );
      //this.state.lastElemet = this.state.valueAchievements.find(this.state.valueTime > this.state.valueAchievements.)
    }
    if (this.state.authorized) {return e ("div", {className: "container py-5"},
      e ("div", {className: "row"}, 
        e ("div", {className: "col-sm-12 col-lg-3 py-5 text-center text-lg-left"}, 
          e ("h2", null, "Hello ", this.state.valueName), 
          e ("p", null, "You are not smoking for: "),
          e ("p", {className: "my-0"}, duration(this.state.valueTime)),
          e ("p", null, "with ", attempts(this.state.valueAttempts)),
          e ("p", null, "Keep it up!"),
          e ("button", {className: "btn btn-warning mb-3",value: "newattempt", onClick: this.handleClickNewAttempt}, "I've smoked :("),
          e ("form", { method: "get"},
            e ("button", {formAction: "logout",type: "submit",className: "btn btn-outline-secondary",value: "logout"}, "Log Out")
          )
        ),
        e ("div", {className: "col-sm-12 col-lg-9 text-justify"},
          e ("div", {style: {height: 340 + 'px', display: 'block', overflow: 'scroll'}},
            e ("h1", {className: ""}, this.state.valueFactTitle),
            e ("p", {className: ""}, this.state.valueFactBody)
          ), e ("div", {className: "py-5"},
            e ("a", {href: this.state.valueFactLink, className: ""}, "Source"),
            e ("button", {className: "float-right btn btn-warning",value: "nextfact", onClick: this.handleClickNextFact}, "Next")
          )
        )
      ),
      e ("div", {className: "row"}, 
        e ("div", {className: "col-sm-12 text-center text-lg-right"},
          e ("h2", null, "Achievements"),
          e ("div", {className: "row", style: {overflowX: 'scroll', display: 'block', whiteSpace: 'nowrap'}}, achievements)
        ) 
      )
    );}
  }
}

const domContainer = document.querySelector('#app');
ReactDOM.render(e(App), domContainer);


function completed(dur, time) {
  if (dur > time) {
    return duration(dur-time);
  } else {
    return "Completed";
  }
}

function attempts(int) {
  if (int > 1) {return int + " attempts";} else {return int + " attempt";}
}

function seconds(time) {
  var label = 'second';
  if (time > 1) {label = 'seconds';}
    return time + ' ' + label;
}
function minutes(time) {
  var label = 'minute';
  if (time >= 120) {label = 'minutes';}
  if (time % 60 !== 0) {
    return Math.floor(time/60)+' '+label + ' ' +seconds(time % 60);
  } else {return Math.floor(time/60)+' '+label;}
}
function hours(time) {
  var label = 'hour';
  if (time >= 7200) {label = 'hours';}
  if (time % 3600 !== 0) {
    if (time < 3660) {return Math.floor(time/3600)+' '+label;}
    else {return Math.floor(time/3600)+' '+label + ' ' +minutes_wo_sec(time % 3600);}
  } else {return Math.floor(time/3600)+' '+label;}
}
function days(time) {
  var label = 'day';
  
  if (time > 172800) {label = 'days';}
  if (time % 86400 !== 0) {
    if (time < 90000) {return Math.floor(time/86400)+' '+label;}
    else {return Math.floor(time/86400)+' '+label + ' ' +hours_wo_min(time % 86400);}
  } else {return Math.floor(time/86400)+' '+label;}
}
function minutes_wo_sec(time) {
  var label = 'minute';
  
  if (time/60 >= 2) {label = 'minutes';}
  return Math.floor(time/60)+' '+label;
}
function hours_wo_min(time) {
  var label = 'hour';
  
  if (time/3600 >= 2) {label = 'hours';}
  return Math.floor(time/3600)+' '+label;
}

function duration(time) {
  if (time < 60) {
    return seconds(time);
  } else if ((time >= 60) && (time < 3600)) {
    return minutes(time);
  }
  else if ((time >= 3600) && (time < 86400)) {
    return hours(time);
  }
  else if (time >= 86400) {
    return days(time);
  }
}