import Avatar from 'primevue/avatar'
import Button from 'primevue/button'
import Badge from 'primevue/badge'
import ContextMenu from 'primevue/contextmenu';
import Dialog from 'primevue/dialog'
import InputSwitch from 'primevue/inputswitch'
import InputText from 'primevue/inputtext'
import Ripple from 'primevue/ripple';
import Skeleton from 'primevue/skeleton';
import Textarea from 'primevue/textarea'
import Toast from 'primevue/toast';


// Plugins
import ToastService from 'primevue/toastservice';


const RegisterComponents = (app) => {

  // Plugins
  app.use(ToastService)
  app.directive('ripple', Ripple);


  // Components
  app.component('Avatar', Avatar)
  app.component('Button', Button)
    app.component('Badge', Badge)
    app.component('ContextMenu', ContextMenu)
    app.component('Dialog', Dialog)
  app.component('InputSwitch', InputSwitch)
  app.component('InputText', InputText)
  app.component('Skeleton', Skeleton)
  app.component('Textarea', Textarea)
    app.component('Toast', Toast)
}
export { RegisterComponents }
