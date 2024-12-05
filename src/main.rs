trait QuestNotifier {
    fn notify(&self, message: &str);
}

struct MailBox;

impl QuestNotifier for MailBox {
    fn notify(&self, message: &str) {
        println!("MailBox: {}", message);
    }
}

struct ChatBox;

impl QuestNotifier for ChatBox {
    fn notify(&self, message: &str) {
        println!("ChatBox: {}", message);
    }
}

struct QuestManager;

impl QuestManager {
    fn notify<T: QuestNotifier>(&self, notifier: T, message: &str) {
        notifier.notify(message);
    }
}

fn main() {
    let quest_manager = QuestManager;
    let mail_box = MailBox;
    let chat_box = ChatBox;

    quest_manager.notify(mail_box, "You've got mail!");
    quest_manager.notify(chat_box, "You've got a message!");
}
