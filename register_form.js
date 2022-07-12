'use strict';
  const e = React.createElement;
    class Achievements extends React.Component {
        constructor(props) {
            super(props);
            this.state = {valueName: ''};
            this.state = {valuePass: ''};
            this.state = {valueMessage: ''};
        
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
            fetch('/register', {
            method: 'POST',
            body: JSON.stringify({username: this.state.valueName, password: this.state.valuePass}),
            headers: {
              'Content-Type': 'application/json',
            }, 
            
          }).then((response) => {
            return response.json();
          })
          .then((data) => {
            console.log(data);
            if (data.message === 'done') {
              location.href = '/';
            } else {
            this.setState({valueMessage: data.message});
            }
          });
            event.preventDefault();
          }
          handleClickLog(event) {
            fetch('/login', {
              method: 'POST',
              body: JSON.stringify({username: this.state.valueName, password: this.state.valuePass}),
              headers: {
                'Content-Type': 'application/json',
              }, 
              
            }).then((response) => {
              return response.json();
            })
            .then((data) => {
              console.log(data);
              if (data.message === 'done') {
                location.href = '/';
              } else {
              this.setState({valueMessage: data.message});
              }
            });
              event.preventDefault();
          }
        render() {
            
          return e ("form", {class: "form-group", method: "post"}, 
            e ("label", null, "Enter Username:",e ("input", {
              class: "form-control",
              name: "username",
              type: "text",
              placeholder: "Username",
              value: this.state.valueName,
              onChange: this.handleChangeName
            })), e ("label", null, "and Password:", e ("input", {
            class: "form-control",
            name: "password",
            type: "password",
            placeholder: "Password",
            value: this.state.valuePass,
            onChange: this.handleChangePass
          })), e ("div", {
            class: "form-row"
          }, e ("div", {
            class: "col-auto"
          }, e ("button", {
            formaction: "login",
            type: "submit",
            class: "btn btn-outline-secondary",
            value: "login",
            onClick: this.handleClickLog
          }, "Log In")),  e("div", {
            class: "col-auto"
          }, e ("button", {
            formaction: "register",
            type: "submit",
            class: "btn btn-warning",
            value: "register",
            onClick: this.handleClickReg
          }, "Register"))), e ("label", {className: "text-danger"}, this.state.valueMessage));
        }
      }
       const domContainer = document.querySelector('#register_form');
      ReactDOM.render(e(Achievements), domContainer); 

