schema "app" {
}

table "users" {
  schema = schema.app
  column "id" {
    type = int
  }
  column "name" {
    type = varchar(255)
  }
  column "manager_id" {
    type = int
  }

  primary_key {
    columns = [
      column.id
    ]
  }
}

table "comments" {
    schema = schema.app

    column "id" {
        type = int
    }

    column "test" {
        type = varchar(255)
    }

    column "user_id" {
        type = int
    }

    foreign_key "user_id_fk" {
        columns = [column.user_id]
        ref_columns = [table.users.column.id]
        on_delete = CASCADE
        on_update = NO_ACTION
    }

    primary_key {
        columns = [
            column.id
        ]
    }
}




