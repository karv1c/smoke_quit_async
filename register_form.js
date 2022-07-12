'use strict';
fetch('/achievements', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    }, 
    
  }).then((response) => {
    return response.json();
  })
  .then((data) => {
    const b = data.lala;
    const e = React.createElement;
    class Achievements extends React.Component {
        constructor(props) {
            super(props);
            this.state = {valueName: ''};
            this.state = {valuePass: ''};
        
            this.handleChangeName = this.handleChangeName.bind(this);
            this.handleChangePass = this.handleChangePass.bind(this);
            this.handleClickReg = this.handleClickReg.bind(this);
            this.handleClickLog = this.handleClickLog.bind(this);
          }
          
          handleChangeName(event) {
            this.setState({valueName: event.target.value});
          }
          handleChangePass(event) {
            this.setState({valuePass: event.target.value});
          }
        
          handleClickReg(event) {
            //const encodedString = Buffer.from(this.state.valueName + ':'+ this.state.valuePass).toString('base64');
            //const encoded = btoa(this.state.valueName + ':' + this.state.valuePass);
            fetch('http://localhost:8080/achievements', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            }, 
            
          });
          //location.href = '/';
          //alert(this.value.valuePass);
            event.preventDefault();
          }
          handleClickLog(event) {
            const encoded = btoa(this.state.valueName + ':' + this.state.valuePass);
            
            fetch('http://localhost:8080/login', {
            method: 'POST',
            withCredentials: true,
            //body: JSON.stringify({username: this.state.valueName, password: this.state.valuePass}),
            headers: {
              'Content-Type': 'application/json',
              'Authorization': 'Basic '+ encoded,
              'WWW-Authenticate': 'Basic'
            }, 
            
          })
          .then(response => response.json())
          .then(json => console.log(json));
          event.preventDefault();
          }
          /* handleClick() {
            const response = fetch('http://localhost:8080', {
            method: 'POST',
            body: JSON.stringify({
              name: 'John Smith',
              job: 'manager',
            }),
            headers: {
              'Content-Type': 'application/json',
              Accept: 'application/json',
            },
          });
          } */
        render() {
            
          return e ("form", {
            class: "form-group",
            method: "post",
          }, e ("label", null, b, e ("input", {
            class: "form-control",
            name: "username",
            type: "text",
            placeholder: "Username",
            value: this.state.valueName,
            onChange: this.handleChangeName
          })), e ("label", null, "and Password:", e ("input", {
            class: "form-control",
            name: "password",
            type: "text",
            placeholder: "Password",
            value: this.state.valuePass,
            onChange: this.handleChangePass
          })), e ("div", {
            class: "form-row"
          }, e ("div", {
            class: "col-auto"
          }, e ("button", {
            formaction: "register",
            type: "submit",
            class: "btn btn-warning",
            value: "register",
            onClick: this.handleClickReg
          }, "Register")),  e("div", {
            class: "col-auto"
          }, e ("button", {
            formaction: "login",
            type: "submit",
            class: "btn btn-outline-secondary",
            value: "login",
            onClick: this.handleClickLog
          }, "Log In"))));
        }
      }
       const domContainer = document.querySelector('#register_form');
      ReactDOM.render(e(Achievements), domContainer); 
  });

//var b = 'sx';


